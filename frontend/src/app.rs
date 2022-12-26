use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            // <img class="logo" src="https://media3.giphy.com/media/R7AW255ijTdV6/giphy.gif" alt="Joyeux No(no)ël" />
            <div class="logo"> </div>
            <h1>{ "Joyeux Noël ma Nono ❤️" }</h1>
            <span class="subtitle">{ "Fait par ton Dylan avec amour" }</span>
        </main>
    }
}
