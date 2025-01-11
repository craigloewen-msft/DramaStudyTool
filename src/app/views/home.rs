use leptos::{html::Input, logging::log, prelude::*, task::spawn_local};
use leptos::ev::Event;

use crate::ai_interface::{SubtitleTranslationInfo, VocabularyInfo};

use web_sys::HtmlInputElement;

use srtlib::Subtitles;

use std::iter::Iterator;

fn format_timestamp_without_ms(timestamp: &srtlib::Timestamp) -> String {
    let timestamp_vals = timestamp.get();
    format!("{:02}:{:02}:{:02}", timestamp_vals.0, timestamp_vals.1, timestamp_vals.2)
}

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

async fn print_file_content(input: Option<HtmlInputElement>) -> Result<Subtitles, String> {
    let files = input.ok_or("No input element found")?.files()
        .ok_or("No files selected")?;
    let file = files.get(0).ok_or("No file found")?;
    let text_promise = file.text();
    
    let subtitle_text = wasm_bindgen_futures::JsFuture::from(text_promise).await
        .map_err(|e| e.as_string().unwrap_or("Unknown error reading file".to_string()))
        .and_then(|text| text.as_string().ok_or("Could not convert file content to string".to_string()))?;

    Subtitles::parse_from_str(subtitle_text)
        .map_err(|e| e.to_string())
}

#[component]
fn SubtitleFileInput(translate_action: ServerAction<GetTranslateInfo>) -> impl IntoView {
    let file_input: NodeRef<Input> = NodeRef::new();
    let (subtitle_content, set_subtitle_content) = signal(Subtitles::new());
    
    // Add current subtitle index signal
    let (current_subtitle_idx, set_current_subtitle_idx) = signal(0usize);
    
    // Create computed signal for current subtitle
    let current_subtitle = move || {
        subtitle_content.with(|subs| {
            subs.clone().to_vec()
                .get(current_subtitle_idx.get())
                .cloned()
        })
    };

    // Navigation functions
    let move_forward = move |step: usize| {
        let max_idx = subtitle_content.with(|subs| subs.clone().to_vec().len().saturating_sub(1));
        let new_idx = (current_subtitle_idx.get() + step).min(max_idx);
        set_current_subtitle_idx.set(new_idx);
    };

    let move_backward = move |step: usize| {
        let new_idx = current_subtitle_idx.get().saturating_sub(step);
        set_current_subtitle_idx.set(new_idx);
    };

    // Add a new signal for the error message
    let (timestamp_error, set_timestamp_error) = signal(Option::<String>::None);

    // Update the jump_to_time function
    let jump_to_time = move |time_str: String| {
        // Add milliseconds to make it compatible with srtlib::Timestamp
        let time_str_with_ms = format!("{},000", time_str);
        
        match srtlib::Timestamp::parse(&time_str_with_ms) {
            Ok(parsed_time) => {
                set_timestamp_error.set(None);
                subtitle_content.with(|subs| {
                    let subtitles = subs.clone().to_vec();
                    let closest_idx = subtitles.iter()
                        .enumerate()
                        .min_by(|(_, a), (_, b)| {
                            let a_after = a.start_time >= parsed_time;
                            let b_after = b.start_time >= parsed_time;
                            
                            match (a_after, b_after) {
                                (true, true) | (false, false) => a.start_time.cmp(&b.start_time),
                                (true, false) => std::cmp::Ordering::Less,
                                (false, true) => std::cmp::Ordering::Greater,
                            }
                        })
                        .map(|(idx, _)| idx)
                        .unwrap_or(0);
                    
                    set_current_subtitle_idx.set(closest_idx);
                });
            },
            Err(_) => {
                set_timestamp_error.set(Some("Invalid timestamp format. Use HH:MM:SS".to_string()));
            }
        }
    };

    view! {
        <h3>File Upload</h3>
        <input
            type="file"
            accept=".srt"
            node_ref=file_input
            on:change=move |_| {
                let file_input_value = file_input.get();
                spawn_local(async move {
                    match print_file_content(file_input_value).await {
                        Ok(subtitle_output) => {
                            log!("File content: {:?}", subtitle_output);
                            set_subtitle_content.set(subtitle_output);
                            // Reset index when new file is loaded
                            set_current_subtitle_idx.set(0);
                        },
                        Err(e) => {
                            log!("Error reading file: {}", e);
                        }
                    }
                })
            }
        />

        <div class="subtitle-navigation mt-3">
            <div class="subtitle-text mb-3">
                // Show placeholder text when no subtitles
                {move || current_subtitle()
                    .map(|sub| sub.text)
                    .unwrap_or_else(|| "Upload a subtitle file to begin".to_string())}
            </div>

            // Add translate button here
            <button 
                class="btn btn-success mb-3"
                on:click=move |_| {
                    if let Some(subtitle) = current_subtitle() {
                        translate_action.dispatch(GetTranslateInfo {
                            input_text: subtitle.text
                        });
                    }
                }
                // Disable if no subtitles
                prop:disabled=move || subtitle_content.with(|subs| subs.clone().to_vec().is_empty())
            >
                "Translate"
            </button>

            <div class="subtitle-timing mb-2">
                <input
                    type="text"
                    class="form-control"
                    class:is-invalid=move || timestamp_error.get().is_some()
                    style="width: 200px"
                    prop:value=move || current_subtitle()
                        .map(|sub| format_timestamp_without_ms(&sub.start_time))
                        .unwrap_or_default()
                    on:change=move |ev| {
                        let time_str = event_target_value(&ev);
                        jump_to_time(time_str);
                    }
                    // Disable if no subtitles
                    prop:disabled=move || subtitle_content.with(|subs| subs.clone().to_vec().is_empty())
                />
                <Show
                    when=move || timestamp_error.get().is_some()
                    fallback=|| view! {}
                >
                    <div class="invalid-feedback">
                        {move || timestamp_error.get()}
                    </div>
                </Show>
            </div>
            <div class="navigation-buttons">
                <button 
                    class="btn btn-secondary me-2"
                    on:click=move |_| move_backward(10)
                    // Disable if no subtitles
                    prop:disabled=move || subtitle_content.with(|subs| subs.clone().to_vec().is_empty())
                >
                    "⏪ Skip 10"
                </button>
                <button 
                    class="btn btn-primary me-2"
                    on:click=move |_| move_backward(1)
                    prop:disabled=move || subtitle_content.with(|subs| subs.clone().to_vec().is_empty())
                >
                    "◀ Back"
                </button>
                <button 
                    class="btn btn-primary me-2"
                    on:click=move |_| move_forward(1)
                    prop:disabled=move || subtitle_content.with(|subs| subs.clone().to_vec().is_empty())
                >
                    "Forward ▶"
                </button>
                <button 
                    class="btn btn-secondary"
                    on:click=move |_| move_forward(10)
                    prop:disabled=move || subtitle_content.with(|subs| subs.clone().to_vec().is_empty())
                >
                    "Skip 10 ⏩"
                </button>
            </div>
        </div>
    }
}