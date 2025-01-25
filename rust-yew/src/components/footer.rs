use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <p>{ "Rust-Yew App © 2025" }</p>
        </footer>
    }
}
