use leptos::prelude::*;

use crate::model::blog_post::Post;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    view! {
        <div class="block p-10">
            <div class="text-4xl pb-4">{post.title.clone()}</div>
            <div>{post.text.clone()}</div>
        </div>
    }
}
