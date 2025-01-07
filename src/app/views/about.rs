use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <h1>Studying with dramas</h1>
        <div>
            <p>
                I made this site since I am trying to improve my vocabulary and grammar by watching K-dramas.
            </p>

            <p>
                You can search up any words, or phrases, then get a translation.
                The translation, and the vocabulary will then be stored at the bottom of the page.
                You can then download this file, to keep a track of words and phrases you needed help with to review in the future.
            </p>
        </div>
    }
}
