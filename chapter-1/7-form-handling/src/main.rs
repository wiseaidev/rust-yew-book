use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

#[function_component(FormModel)]
pub fn form_model() -> Html {
    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
            }
        })
    };

    let on_password_change = {
        let input_password_ref = input_password_ref.clone();

        Callback::from(move |_| {
            let input = input_password_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_password_handle.set(value);
            }
        })
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        console::log_1(&format!("Email: {}, Password: {}", input_email, input_password).into());

        spawn_local(async move {
            console::log_1(&"success".into());
            let window: Window = web_sys::window().expect("window not available");
            let location = window.location();
            let _ = location.set_href("/profile");
        });
    });

    html! {
        <section
        >
            <div>
                <h2>
                  {"Welcome Back!"}
                </h2>
                <form onsubmit={onsubmit}>
                  <div>
                    <label for="username"
                      >{"Email"}</label
                    >
                      <input
                        type="text"
                        id="email"
                        name="email"
                        placeholder="Email"
                        required={true}
                        ref={input_email_ref}
                        oninput={on_email_change}
                      />
                  </div>
                  <div>
                    <label for="password"
                      >{"Password"}</label
                    >
                    <input
                      type="password"
                      id="password"
                      name="password"
                      placeholder="Password"
                      required={true}
                      ref={input_password_ref}
                      oninput={on_password_change}
                    />
                  </div>
                  <button
                    type="submit"
                  >
                    {"Submit"}
                  </button>
                </form>
            </div>
        </section>
    }
}

fn main() {
    yew::Renderer::<FormModel>::new().render();
}