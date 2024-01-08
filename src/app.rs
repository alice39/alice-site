mod home;

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/alice-site.css"/>

        // sets the document title
        <Title text="#Alice-Corner"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="blog" view=UnderBuildPage/>
                    <Route path="copyright" view=UnderBuildPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn UnderBuildPage() -> impl IntoView {
    view! {
        <div class="w-screen h-screen flex">
            <h1 class="text-4xl font-bold m-auto">"Underbuild this page..."</h1>
        </div>
    }
}
