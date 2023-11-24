use yew::prelude::*;
use stylist::css;

// Define a functional component named Counter
#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_state(|| 0);
    let on_increment = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let on_decrement = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <div class={css!("color: red;")}>
                {"Hello from Stylist, btw!"}
            </div>
            <h2>{"Counter Component"}</h2>
            <button onclick={on_increment}>{"Increment"}</button>
            <p>{"Count: "} {*counter}</p>
            <button onclick={on_decrement}>{"Decrement"}</button>
        </div>
    }
}

// Define another functional component named App
#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Yew Functional Component Example"}</h1>
            <Counter />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}