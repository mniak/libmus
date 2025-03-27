use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}
mod typescript {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(raw_module = "../node_modules/verovio/dist/verovio-toolkit-wasm.js")]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["ts"], catch)]
        pub fn createVerovioModule() -> Result<String, JsValue>;
    }
}

#[component]
fn App() -> Element {
    let mut eval1 = document::eval(
        r#"
            // You can receive messages from Rust to JavaScript with the dioxus.recv function
            let msg = await dioxus.recv();
            
            while (true) {
                console.log("message from rust:", msg);
            }
            "#,
    );
    let mut message = use_signal(|| "message");

    eval1.send("on_start").unwrap();

    rsx! {
        h1 { {message} }
        button {
            onclick: move |event| {
                eval1.send("on_button_click")
                .map_err(|e| println!("Err on button click: {}", e))
                .unwrap();
            },
            "click me!"
        }
    }
}
