use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());
    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }

    let message = (*welcome).clone();

    html! {
        <div>
            <h2 class={"heading"}>{message}</h2>
        </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
            }
            Err(e) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    });
}