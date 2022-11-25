use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{
    navbar::Navbar,
};
mod pages;
use pages::{
    home::Home, characters::Characters, weapons::Weapons, artifacts::Artifacts, not_found::NotFound,
};

#[derive(Clone, Routable, PartialEq)]
enum Route{
    #[at("/")]
    Home,
    #[at("/characters")]
    Characters,
    #[at("/weapons")]
    Weapons,
    #[at("/artifacts")]
    Artifacts,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(routes: &Route) -> Html{
    match routes{
        Route::Home => html!{<Home />},
        Route::Characters => html!{<Characters />},
        Route::Weapons => html!{<Weapons />},
        Route::Artifacts => html!{<Artifacts />},
        Route::NotFound => html!{<NotFound/>},
    }
}

#[function_component(App)]
pub fn app() -> Html{
    html! {
        <div class="container">
            <Navbar />
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}
fn main() {
    yew::start_app::<App>();
}