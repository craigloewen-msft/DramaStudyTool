use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, MetaTags, Script, Title};
use leptos_router::{
    components::{Route, Router, Routes}, hooks::use_location,
    path,
};

mod views;

use views::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-bs-theme="dark" >
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <Link
                    href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"
                    rel="stylesheet"
                    integrity="sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH"
                    crossorigin="anonymous"
                />
            </head>
            <body>
                <App/>
            </body>
            <Script
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"
                integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz"
                crossorigin="anonymous"
            ></Script>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Stylesheet id="leptos" href="/pkg/dramastudytool.css"/>

        // sets the document title
        <Title text="Drama Study Tool" />

        // content for this welcome page
        <Router>
                <nav>
                    <div class="container">
                        <header class="d-flex flex-wrap justify-content-center py-3 mb-4 border-bottom">
                            <a
                                href="/"
                                class="d-flex align-items-center mb-3 mb-md-0 me-md-auto link-body-emphasis text-decoration-none"
                            >
                                <svg class="bi me-2" width="40" height="32">
                                    <use xlink:href="#bootstrap"></use>
                                </svg>
                                <span class="fs-4">Drama Study Tool</span>
                            </a>
                            <NavLink to=format!("/")>"Home"</NavLink>
                            <NavLink to=format!("/about")>"About"</NavLink>
                        </header>
                    </div>
                </nav>
                <main>
                    <div class="container">
                        <Routes fallback=|| {
                            view! {
                                <h3>404 error!</h3>
                                <p>No page was found for this url.</p>
                            }
                        }>
                            <Route path=path!("/") view=home::Home />
                            <Route path=path!("/about") view=about::About />
                        </Routes>
                    </div>
                </main>
            </Router>
    }
}

// Nav link for navbar
#[component]
fn NavLink(to: String, children: Children) -> impl IntoView {
    let location = use_location();
    let input_path = to;

    view! {
        <ul class="nav nav-pills">
            <li class="nav-item">
                <a
                    href=input_path.clone()
                    class="nav-link"
                    class:active=move || location.pathname.get() == input_path
                    aria-current="page"
                >
                    {children()}
                </a>
            </li>
        </ul>
    }
}