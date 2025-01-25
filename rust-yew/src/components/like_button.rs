use yew::prelude::*;

#[function_component(LikeButton)]
pub fn like_button() -> Html {
    let count = use_state(|| 0);

    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    html! {
        <div>
            <p>{ format!("Likes: {}", *count) }</p>
            <button {onclick}>{ "Like" }</button>
        </div>
    }
}
