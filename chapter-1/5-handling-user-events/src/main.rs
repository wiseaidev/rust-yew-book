use yew::prelude::*;

#[function_component(App)]
fn app() -> Html { // <1>
    let clicked = use_state(|| false); // <2>

    let on_click = {
        let clicked = clicked.clone();
        move |_| {
            clicked.set(!*clicked);
        }
    };

    html! {
        <div>
            <button onclick={on_click}>{ "Toggle Click" }</button> // <3>
            <p>{ if *clicked { "Clicked!" } else { "Not Clicked" } }</p> // <4>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // <5>
}