use yew::{Component, Html};
use yew::macros::html;

pub trait ComponentDemo: Component {
    fn demo() -> Vec<Self>;

    fn demo_html() -> Html {
        let components: Vec<Html> = Self::demo()
            .into_iter()
            .map(|c|
                html! {
                    <div>
                        { c.view() }
                        <br /><br />
                    </div>
                }
            )
            .collect();
        html! { <div>{ components }</div> }
    }
}