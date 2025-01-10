use leptos::{html::A, prelude::*};
use server_fn::ServerFn;

use crate::ai_interface::{GrammarPointInfo, SubtitleTranslationInfo, VocabularyInfo};

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

    let (saved_word_list, set_saved_word_list) = signal(vec![(0 as usize, VocabularyInfo{ word: "Dog".to_string(), translation: "Is a dog".to_string() })]);

    let remove_saved_word_fn = move |index: usize| { 
        let mut saved_word_list_value = saved_word_list.get();
        saved_word_list_value.remove(index);
        set_saved_word_list.set(saved_word_list_value)
     };

    view! {
        <div class="row">
            <div class="col col-md-6">
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
            </div>
            <div class="col col-md-6">
                <TranslationBox translate_action=get_translate_info_action />
            </div>
        </div>
        <div class="row">
            <SavedWordBox saved_word_list=saved_word_list />
            <button on:click=move |_| remove_saved_word_fn(0)>Remove</button>
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
fn TranslationBox(translate_action: ServerAction<GetTranslateInfo>) -> impl IntoView {

    let translation_pending = translate_action.pending();

    let translation_result_option = translate_action.value();

    let get_translation_object = move || translation_result_option.get().unwrap().unwrap();

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
                                view! {
                                    <li>
                                        <b>{vocab.word}</b>
                                        -
                                        {vocab.translation}
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
fn SavedWordBox(saved_word_list: ReadSignal<Vec<(usize, VocabularyInfo)>>) -> impl IntoView {
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
                    </li>
                }
            }
        />
    }
}