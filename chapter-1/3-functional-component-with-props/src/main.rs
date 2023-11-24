use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GreetingProps {
    pub message: String,
}

#[function_component(Greeting)]
fn greeting(props: &GreetingProps) -> Html {
    html! {
        <div>
            <h1>{&props.message}</h1>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Greeting message={"Hello, YEW!".to_string()} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}