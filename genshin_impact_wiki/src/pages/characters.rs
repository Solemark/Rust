use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct Character{
    name: String,
    title: String,
    vision: String,
    weapon: String,
    nation: String,
    affiliation: String,
    rarity: i8,
    constellation: String,
    birthday: String,
    description: String
}


#[function_component(Characters)]
pub fn characters() -> Html{

    let characters: UseStateHandle<Vec<Character>> = use_state(|| vec![]);
    {
        let characters: UseStateHandle<Vec<Character>> = characters.clone();
        use_effect_with_deps(move |_| {
            let characters: UseStateHandle<Vec<Character>> = characters.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let data: Vec<Character> = Request::get("https://api.genshin.dev/characters/all")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                characters.set(data);
            });
            || ()
        }, ());
    }
    html!(
        <div>
            {"Character Page"}
        </div>
    )
}