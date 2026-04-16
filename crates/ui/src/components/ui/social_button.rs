use sycamore::prelude::*;

#[derive(Props)]
pub struct SocialButtonProps {
    icon: View,
    label: String,
    on_click: Box<dyn FnMut()>,
}

#[component]
pub fn SocialButton(mut props: SocialButtonProps) -> View {
    view! {
        button(
            r#type="button",
            class="social-btn",
            on:click=move |_| (*props.on_click)(),
        ) {
            (props.icon)
            (props.label)
        }
    }
}
