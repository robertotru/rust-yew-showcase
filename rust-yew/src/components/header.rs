use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>{ "Welcome to Rust-Yew App" }</h1>
        </header>
    }
}
