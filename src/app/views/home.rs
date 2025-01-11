use leptos::{ev::{Event, MouseEvent, SubmitEvent}, html::{Input, A}, logging::log, prelude::*, task::spawn_local};
use server_fn::ServerFn;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use crate::ai_interface::{GrammarPointInfo, SubtitleTranslationInfo, VocabularyInfo};

use web_sys::{Blob, FileReader, HtmlInputElement};

#[server]
pub async fn get_translate_info(input_text: String) -> Result<SubtitleTranslationInfo, ServerFnError> {
    use crate::ai_interface::*;
    use leptos::logging::*;
    // use crate::context::DramaStudyToolAppContext;
    // let app_context_option = use_context::<DramaStudyToolAppContext>();
    // let app_context = match app_context_option {
    //     Some(context) => context,
    //     None => return Err(ServerFnError::ServerError(format!("Couldn't get app context"))),
    // };
    // let ai_interface = &app_context.ai_interface;

    if input_text == "".to_string() {
        return Err(ServerFnError::new(format!("Input text is empty")));
    }

    let translated_text_info = match AIInterface::translate(input_text.clone()).await {
        Ok(translated_text) => translated_text,
        Err(e) => return Err(ServerFnError::new(format!("Error: {:?}", e))),
    };

    Ok(translated_text_info)
}

#[component]
pub fn Home() -> impl IntoView {
    let get_translate_info_action = ServerAction::<GetTranslateInfo>::new();

    let (saved_word_list, set_saved_word_list) = signal(vec![
        (0 as usize, VocabularyInfo{ word: "Dog".to_string(), translation: "Is a dog".to_string() }),
        (1 as usize, VocabularyInfo{ word: "Cat".to_string(), translation: "Is a cat".to_string() }),
    ]);

    let (direct_input, direct_input_set) = signal(true);

    view! {
        <div class="row">
            <div class="col col-md-6">
                <ul class="nav nav-tabs">
                    <li class="nav-item">
                        <a
                            class="nav-link"
                            class:active=move || direct_input.get()
                            on:click=move |_| { direct_input_set.set(true) }
                            aria-current="page"
                        >
                            Text Input
                        </a>
                    </li>
                    <li class="nav-item">
                        <a
                            class="nav-link"
                            class:active=move || !direct_input.get()
                            on:click=move |_| { direct_input_set.set(false) }
                            aria-current="page"
                        >
                            Upload
                        </a>
                    </li>
                </ul>
                <Show
                    when=move || direct_input.get()
                    fallback=move || {
                        view! { <SubtitleFileInput translate_action=get_translate_info_action /> }
                    }
                >
                    <ActionForm action=get_translate_info_action>
                        <div class="mb-3">
                            <label for="translation_input" class="form-label">
                                Input language text
                            </label>
                            <input
                                class="form-control"
                                type="text"
                                placeholder="e.g: 사전을 못 찾아"
                                name="input_text"
                            />
                        </div>
                        <input type="submit" class="btn btn-primary" value="Translate" />
                    </ActionForm>
                </Show>
            </div>
            <div class="col col-md-6">
                <TranslationBox
                    translate_action=get_translate_info_action
                    saved_word_list=saved_word_list
                    set_saved_word_list=set_saved_word_list
                />
            </div>
        </div>
        <div class="row">
            <hr />
            <SavedWordBox saved_word_list=saved_word_list set_saved_word_list=set_saved_word_list />
        </div>
    }
}

#[component]
fn TranslationOutputBox(message: String) -> impl IntoView {
    view! {
        <div class="translation-output-box">
            <h4>{{ message }}</h4>
        </div>
    }
}

#[component]
fn TranslationBox(translate_action: ServerAction<GetTranslateInfo>, saved_word_list: ReadSignal<Vec<(usize, VocabularyInfo)>>, set_saved_word_list: WriteSignal<Vec<(usize,VocabularyInfo)>>) -> impl IntoView {

    let translation_pending = translate_action.pending();

    let translation_result_option = translate_action.value();

    let get_translation_object = move || translation_result_option.get().unwrap().unwrap();

     let add_saved_word_fn = move |new_element: VocabularyInfo| { 
        let mut saved_word_list_value = saved_word_list.get();
        let max_index = match saved_word_list_value.last() {
            Some((index, _)) => index + 1,
            None => 0,
        };
        saved_word_list_value.push((max_index, new_element));
        set_saved_word_list.set(saved_word_list_value)
     };

    // let fake_data = SubtitleTranslationInfo {
    //     translation: "I love dogs.".to_string(),
    //     vocabulary: vec![
    //         VocabularyInfo {
    //             word: "Dog".to_string(),
    //             translation: "It's a dog".to_string(),
    //         },
    //     ],
    //     grammar_points: vec![
    //         GrammarPointInfo {
    //             name: "Verb".to_string(),
    //             relevant_text: "love".to_string(),
    //             description: "Verbing is verb".to_string(),
    //         }
    //     ]
    // };
    // let translation_result_option = RwSignal::new(Some(Ok::<SubtitleTranslationInfo,ServerFnError>(fake_data)));
    // // let translation_result_option = RwSignal::new(Some(Err::<SubtitleTranslationInfo,ServerFnError>(ServerFnError::new("Broken"))));

    
    view! {
        <p class="translation-output-label">Translation Output:</p>
        <Show
            when=move || !translation_pending.get()
            fallback=|| view! { <TranslationOutputBox message="Translating...".to_string() /> }
        >
            <Show
                when=move || translation_result_option.get().is_some()
                fallback=|| {
                    view! {
                        <TranslationOutputBox message="Translation output goes here".to_string() />
                    }
                }
            >
                <Show
                    when=move || translation_result_option.get().unwrap().is_ok()
                    fallback=move || {
                        view! {
                            <div class="alert alert-danger">
                                {translation_result_option.get().unwrap().unwrap_err().to_string()}
                            </div>
                            <TranslationOutputBox message="Error".to_string() />
                        }
                    }
                >
                    <TranslationOutputBox message=get_translation_object().translation />
                    <h4>Vocabulary</h4>
                    <ul>
                        {get_translation_object()
                            .vocabulary
                            .into_iter()
                            .map(|vocab| {
                                let vocab_clone = vocab.clone();
                                view! {
                                    <li>
                                        <b>{vocab.word}</b>
                                        -
                                        {vocab.translation}
                                        <button on:click=move |_| add_saved_word_fn(
                                            vocab_clone.clone(),
                                        )>Add word</button>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                    <h4>Grammar</h4>
                    <ul>
                        {get_translation_object()
                            .grammar_points
                            .into_iter()
                            .map(|grammar| {
                                view! {
                                    <li>
                                        <b>{grammar.name}</b>
                                        -
                                        {grammar.description}
                                        <br />
                                        {grammar.relevant_text}
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </Show>
            </Show>
        </Show>
    }
}

#[component]
fn SavedWordBox(saved_word_list: ReadSignal<Vec<(usize, VocabularyInfo)>>, set_saved_word_list: WriteSignal<Vec<(usize,VocabularyInfo)>>) -> impl IntoView { 
 let remove_saved_word_fn = move |index: usize| { 
        let mut saved_word_list_value = saved_word_list.get();
        saved_word_list_value.retain(|(id,_)| { id != &index });
        set_saved_word_list.set(saved_word_list_value)
     };

    view! {
        <h4>Saved words</h4>
        <For
            each=move || saved_word_list.get()
            key=|vocab_entry| vocab_entry.0
            children=move |(id, vocab)| {
                view! {
                    <li>
                        <b>{vocab.word}</b>
                        -
                        {vocab.translation}
                        <button on:click=move |_| remove_saved_word_fn(id)>Remove</button>
                    </li>
                }
            }
        />
    }
}

async fn PrintFileContent(input: Option<HtmlInputElement>) {
        log!("Input: {:?}", input);
        let value = input.unwrap().files();
        log!("files: {:?}", value);
        let value_unwrapped = value.unwrap();
        let get_file = value_unwrapped.get(0);
        log!("File option: {:?}", get_file);
        let file_text = get_file.unwrap().text();
        log!("File text: {:?}", file_text);
        let result = wasm_bindgen_futures::JsFuture::from(file_text).await;
        log!("Result: {:?}", result);
}

#[component]
fn SubtitleFileInput(translate_action: ServerAction<GetTranslateInfo>) -> impl IntoView {
    let file_input: NodeRef<Input> = NodeRef::new();

    view! {
        <h3>File Upload</h3>
        <input
            type="file"
            node_ref=file_input
            on:change=move |e| {
                let file_input_value = file_input.get();
                spawn_local(async move {
                    PrintFileContent(file_input_value).await;
                })
            }
        />
    }
}