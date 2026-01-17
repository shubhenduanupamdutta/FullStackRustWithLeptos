use crate::component::{blog_previews::BlogPreviews, edit_post::EditPost, view_post::ViewPost};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <h2> Blog </h2>
        <nav>
            <ul>
                <li><a href="/">Blog</a></li>
                <li><a href="/edit">Create</a></li>
            </ul>
        </nav>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>

        <Navbar />
        <Router>
            <main>
                <Routes fallback=|| "404 Not Found">
                    <Route path=path!("") view=BlogPreviews />
                    <Route path=path!("edit/:post_id?") view=EditPost />
                    <Route path=path!("view/:post_id?") view=ViewPost />
                </Routes>
            </main>
        </Router>
    }
}
