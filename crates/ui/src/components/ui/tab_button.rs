use sycamore::prelude::*;

#[derive(Props)]
pub struct TabButtonProps {
    label: String,
    on_click: Box<dyn FnMut()>,
    is_active: Box<dyn Fn() -> bool>,
}

#[component]
pub fn TabButton(mut props: TabButtonProps) -> View {
    view! {
        button(
            class=if (*props.is_active)() { "tab-btn active" } else { "tab-btn" },
            on:click=move |_| (*props.on_click)(),
        ) {
            (props.label)
        }
    }
}
