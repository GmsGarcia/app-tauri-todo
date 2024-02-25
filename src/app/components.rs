use yew::prelude::*;
use yew_lucide::PlusSquare;

#[derive(Clone, PartialEq, Properties)]
pub struct TodoProps {
    pub title: String,
    pub description: String
}

#[function_component]
pub fn TodoItem(props: &TodoProps) -> Html {
    let TodoProps { title, description } = props;

    html! {
        <div class="item">
            <h1>{ title }</h1>
            <h3>{ description }</h3>
            <svg class="icon" viewBox="0 0 24 24">
                <rect width="18" ry="2" height="18" x="3" rx="2" y="3"  /><line y1="8" x1="12" y2="16" x2="12"  /><line x1="8" y1="12" y2="12" x2="16"  />
            </svg>
        </div>
    }
}
