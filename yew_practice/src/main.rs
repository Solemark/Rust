use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html{
    let counter: UseStateHandle<i32> = use_state(|| 0);

    let increment: Callback<MouseEvent> = {
        let counter: UseStateHandle<i32> = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let decrement: Callback<MouseEvent> = {
        let counter: UseStateHandle<i32> = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };

    html!(
        <div>
            <p>{*counter}</p>
            <button type="button" class="btn btn-primary" onclick={increment}>{"Increment"}</button>
            <button type="button" class="btn btn-primary" onclick={decrement}>{"Decrement"}</button>
        </div>
    )
}
fn main() {
    yew::start_app::<Counter>();
}