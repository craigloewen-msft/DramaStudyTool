use leptos::{prelude::*, task::spawn_local};

// #[server]
// pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
//     leptos::logging::log!("Called it with value: {}", title);
// }

#[component]
pub fn Home() -> impl IntoView {
    let (translation_input, set_translation_input) = signal("".to_string());

    view! {
        <div class="row">
            <div class="col col-md-6">
                <div class="mb-3">
                    <label for="translation_input" class="form-label">
                        Input language text
                    </label>
                    <input
                        class="form-control"
                        id="translation_input"
                        type="text"
                        placeholder="e.g: 사전을 못 찾아"
                        bind:value=(translation_input, set_translation_input)
                    />
                </div>
                <button
                    class="btn btn-primary"
                    // on:click=move |_| {
                    //     spawn_local(async {
                    //         let _ = add_todo("So much to do!".to_string()).await;
                    //     });
                    // }
                >
                    Submit
                </button>
            </div>
            <div class="col col-md-6">
                <p class="translation-output-label">Translation Output:</p>
                <div class="translation-output-box">
                    <h3>
                        {move || {
                            if translation_input.get() == "".to_string() {
                                "Output will go here".to_string()
                            } else {
                                translation_input.get()
                            }
                        }}
                    </h3>
                </div>
            </div>
        </div>
        <div class="row">
            <div>{translation_input}</div>
        </div>
    }
}
