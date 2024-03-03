use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod components;
use components::TodoItemComponent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SearchArgs<'a> {
    input: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let search_input_ref = use_node_ref();

    let ipt = use_state(|| String::new());

    let search_val = use_state(|| String::new());
    {
        let search_val = search_val.clone();
        let ipt = ipt.clone();
        let ipt2 = ipt.clone();
        use_effect_with(ipt2, move |_| {
            spawn_local(async move {
                if ipt.is_empty() {
                    return;
                }

                let args = to_value(&SearchArgs { input: &*ipt }).unwrap();
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                let search_res = invoke("search", args).await.as_string().unwrap();
                search_val.set(search_res);
            });

            || {}
        });
    }

    let search = {
        let ipt = ipt.clone();
        let search_input_ref = search_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            ipt.set(
                search_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <form class="search" onsubmit={ search }>
                <input
                ref={ search_input_ref }
                type="text"
                name="search"
                id="ipt_search"
                placeholder="Search Todo by name/title:" />
                <button type="submit">{ "Add Todo" }</button>
            </form>

            <p><b>{ &*search_val }</b></p>

            <div class="todos">
                <TodoItemComponent title={ "Shop list" } description={ "Cheese, Ham, Bread, Milk, Pizza"} />
                <TodoItemComponent title={ "Project TODO" } description={ "Fix UI, Handle HTTP requests, Authenticate system" }/>
                <TodoItemComponent title={ "Gardening tools" } description={ "Rake, Shovel, Gloves, Watering Can"} />
                <TodoItemComponent title={ "My Passwords" } description={ "password123, isTh1sS4fe" } />
                <TodoItemComponent title={ "secret list" } description={ "he deals the card to find the answer, the sacred geometry of change, the hidden law of probable outcome, the numbers lead the dance"} />
            </div>
        </main>
    }
}
