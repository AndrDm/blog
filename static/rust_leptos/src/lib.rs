// use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ---- your component ----
#[component]
pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(initial_value);

    let clear = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|v| *v -= 1);
    let increment = move |_| set_value.update(|v| *v += 1);

    view! {
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>" Value: " {value} " "</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

//  WASM entry point (IMPORTANT)
#[wasm_bindgen]
pub fn start(container_id: &str) {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
let root = document
    .get_element_by_id(container_id)
    .expect("container not found")
    .dyn_into::<web_sys::HtmlElement>()
    .expect("not an HtmlElement");

    mount_to(root, || view! {
        <SimpleCounter initial_value=5 />
    });
}
