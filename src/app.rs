use leptonic::components::prelude::*;
use leptonic::prelude::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, Meta, Title};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    tracing::info!("Welcome to Leptonic");

    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let greet = move || {
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let (count, set_count) = signal(0);

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic Tauri template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Title text="Leptonic Tauri template"/>

        <Root default_theme=LeptonicTheme::default()>
            <div style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
                <h2>"Welcome to Leptonic"</h2>

                <Stack spacing=Size::Em(2.0)>
                    <div style="width: 100%;">
                        <div style="margin-top: 3em;">"Count: " {move || count.get()}</div>
                        <Button on_press=move|_| set_count.update(|c| *c += 1)>
                            "Increase"
                        </Button>
                    </div>

                    <div style="width: 100%;">
                        <TextInput get=name set=set_name placeholder=Oco::Borrowed("Enter a name...")/>
                        <Button on_press=move|_| greet()>
                            "Greet"
                        </Button>
                        <p><b>{ move || greet_msg.get() }</b></p>
                    </div>
                </Stack>
            </div>
        </Root>
    }
}
