use reqwest::Client;
use yew::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct BlogPost {
    id: i32,
    title: String,
    content: String,
    images: Vec<String>,
}

#[function_component(Slider)]
fn slider(props: &SliderProps) -> Html {
    let index = use_state(|| 0);
    let images = props.images.clone();
    let len = images.len();

    let prev = {
        let index = index.clone();
        Callback::from(move |_| {
            let new_index = if *index == 0 { len - 1 } else { *index - 1 };
            index.set(new_index);
        })
    };

    let next = {
        let index = index.clone();
        Callback::from(move |_| {
            let new_index = if *index == len - 1 { 0 } else { *index + 1 };
            index.set(new_index);
        })
    };

    html! {
        <div class="slider-container">
            <button class="slider-button left" onclick={prev}>{"<"}</button>
            <div class="slider">
                <img class="slider-image" src={images[*index].clone()} alt="Post Image" />
            </div>
            <button class="slider-button right" onclick={next}>{">"}</button>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct SliderProps {
    images: Vec<String>,
}

#[function_component(BlogPostComponent)]
fn blog_post_component() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();
        use_effect_with((), move |_: &()| { // Correct closure signature with &
            wasm_bindgen_futures::spawn_local(async move {
                let client = Client::new();
                let fetched_posts: Vec<BlogPost> = client
                    .get("http://127.0.0.1:8888/posts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                posts.set(fetched_posts);
            });
        });
    }

    html! {
        <div class="blog-posts">
            { for (*posts).iter().map(|post| html! {
                <div class="blog-post">
                    <h2>{&post.title}</h2>
                    <p>{&post.content}</p>
                    <Slider images={post.images.clone()} />
                </div>
            })}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <BlogPostComponent />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

