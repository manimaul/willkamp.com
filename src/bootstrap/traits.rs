use yew::{Component, Html};
use yew::macros::html;

pub trait ComponentDemo<T: Component> {
    fn demo() -> Vec<T>;

    fn demo_html() -> Html {
        let components: Vec<Html> = Self::demo()
            .into_iter()
            .map(|badge|
                html! {
                    <div>
                        { badge.view() }
                        <br />
                    </div>
                }
            )
            .collect();
        html! { <div>{ components }</div> }
    }
}