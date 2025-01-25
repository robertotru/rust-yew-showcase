use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <components::Header />
            <components::LikeButton />
            <components::Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}