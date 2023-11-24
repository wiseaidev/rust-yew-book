use yew::prelude::*;
use web_sys::{HtmlInputElement};
use std::collections::HashMap;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

#[function_component(StateModel)]
pub fn state_model() -> Html {
    // Node references and state variables initialization
    let input_key_ref = use_node_ref();
    let input_key_handle = use_state(String::default);
    let input_key = (*input_key_handle).clone();

    let input_value_ref = use_node_ref();
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    // Retrieving existing data from local storage
    let storage_data: HashMap<String, String> = LocalStorage::get("data").unwrap_or_default();

    // State variable for managing data
    let data_handle = use_state(|| storage_data);
    let data = (*data_handle).clone();
    let data_values = (*data_handle).clone();

    // Callbacks for handling input changes
    let on_key_change = {
        let input_key_ref = input_key_ref.clone();
        let input_key_handle = input_key_handle.clone();

        Callback::from(move |_| {
            let input = input_key_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_key_handle.set(value);
            }
        })
    };

    let on_value_change = {
        let input_value_ref = input_value_ref.clone();
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |_| {
            let input = input_value_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_value_handle.set(value);
            }
        })
    };

    // Callback for form submission
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let input_ref = input_key.clone();
        let mut data = data.clone();
        let input_key_handle = input_key_handle.clone();
        let input_value_handle = input_value_handle.clone();
        if !input_ref.is_empty() {
            data.insert(input_key.clone(), input_value.clone());
            data_handle.set(data.clone());

            // Persisting data in local storage
            LocalStorage::set("data", Ok::<HashMap<String, String>, String>(data.clone())).ok();

            // Clearing input fields
            input_key_handle.set(String::default());
            input_value_handle.set(String::default());
        }
    });

    html! {
        <section
        >
            <div>
                <h2>
                  {"Yew Local Storage."}
                </h2>
                <form onsubmit={onsubmit}>
                  <div>
                    <label for="key"
                      >{"key"}</label
                    >
                      <input
                        type="text"
                        id="key"
                        name="key"
                        placeholder="Key"
                        required={true}
                        ref={input_key_ref}
                        oninput={on_key_change}
                      />
                  </div>
                  <div>
                    <label for="value"
                      >{"Value"}</label
                    >
                    <input
                      type="text"
                      id="value"
                      name="value"
                      placeholder="Value"
                      required={true}
                      ref={input_value_ref}
                      oninput={on_value_change}
                    />
                  </div>
                  <button
                    type="submit"
                  >
                    {"Set Data"}
                  </button>
                </form>
                <ul>
                    { for data_values.iter().map(|(key, value)| {
                        html! {
                            <li>{ key } {":"} { value }</li>
                        }
                    }) }
                </ul>
            </div>
        </section>
    }
}

fn main() {
    yew::Renderer::<StateModel>::new().render();
}