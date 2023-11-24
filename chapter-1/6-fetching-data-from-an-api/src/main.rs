use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[function_component(DataModel)]
fn data_model() -> Html {
    let posts = use_state(|| vec![]);

    let fetch_data = {
        let posts = posts.clone();
        Callback::from(move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<Post> = Request::get("https://jsonplaceholder.typicode.com/posts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                posts.set(fetched_posts);
            });
        })
    };

    html! {
        <div>
            <button onclick={fetch_data}>{ "Fetch Data" }</button>
            <ul>
                { for posts.iter().map(render_post) }
            </ul>
        </div>
    }
}

fn render_post(post: &Post) -> Html {
    html! {
        <li>
            <h2>{ &post.title }</h2>
            <p>{ &post.body }</p>
        </li>
    }
}


fn main() {
    yew::Renderer::<DataModel>::new().render(); // <5>
}