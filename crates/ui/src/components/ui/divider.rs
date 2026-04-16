use sycamore::prelude::*;

#[derive(Props)]
pub struct DividerProps {
    text: String,
}

#[component]
pub fn Divider(props: DividerProps) -> View {
    view! {
        div(class="divider") {
            span { (props.text) }
        }
    }
}
