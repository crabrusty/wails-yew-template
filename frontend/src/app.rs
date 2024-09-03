use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::{console, HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::prelude::*;

#[wasm_bindgen(module = "/wailsjs/go/main/App.js")]
extern "C" {
    fn Greet(arg1: &str) -> js_sys::Promise;
}

#[function_component(App)]
pub fn app() -> Html {
    let name = use_state(|| String::from(""));
    let greeting = use_state(|| String::from(""));

    // Handle input changes
    let oninput = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            name.set(input.value());
        })
    };

    // Handle button clicks
    let onclick = {
        let name = name.clone();
        let greeting = greeting.clone();
        Callback::from(move |_: MouseEvent| {
            // Accept MouseEvent for button click
            let name_value = name.to_string();
            let greeting = greeting.clone();
            let promise = Greet(&name_value);
            let future = wasm_bindgen_futures::JsFuture::from(promise);

            wasm_bindgen_futures::spawn_local(async move {
                match future.await {
                    Ok(result) => {
                        let greet_message = result.as_string().unwrap_or_default();
                        greeting.set(greet_message);
                    }
                    Err(error) => {
                        console::log_1(&error);
                    }
                }
            });
        })
    };

    // Handle Enter key presses
    let onkeypress = {
        let onclick = onclick.clone(); // Reuse the existing `onclick` callback
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                onclick.emit(MouseEvent::new("click").unwrap()); // Create a MouseEvent to pass to onclick
            }
        })
    };

    html! {
        <div id="app">
            <p> {"Wails + Yew"} </p>
            <a href="https://wails.io">
                <img src="images/wails-logo.svg" class="logo wails" alt="Wails logo"/>
            </a>
            <a href="https://yew.rs">
                <img src="images/yew-logo.svg" class="logo yew" alt="Yew logo"/>
            </a>
            <div class="input-box">
                <input {oninput} {onkeypress} type="text" placeholder="Enter your name" class="input" />
                <button {onclick} class="btn">{ "Greet from Wails" }</button>
            </div>
            <p class="result">{ &*greeting }</p>
        </div>
    }
}
