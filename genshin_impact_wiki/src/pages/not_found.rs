use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html{
    html!(
        <div>
            <h1>{"404"}</h1>
            <br />
            <p>{"Page not found!"}</p>
        </div>
    )
}