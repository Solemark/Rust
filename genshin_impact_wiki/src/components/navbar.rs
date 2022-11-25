use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html{
    html!(
        <nav class="navbar navbar-expand-lg bg-light">
            <div class="container-fluid">
                <a class="navbar-brand" href="/">{"Genshin Impact Wiki"}</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                <ul class="navbar-nav">
                    <li class="nav-item">
                        <a class="nav-link active" aria-current="page" href="/characters">{"Characters"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link active" href="/weapons">{"Weapons"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link active" href="/artifacts">{"Artifacts"}</a>
                    </li>
                </ul>
                </div>
            </div>
        </nav>
    )
}