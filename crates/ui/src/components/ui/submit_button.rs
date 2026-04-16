use sycamore::prelude::*;

#[derive(Props)]
pub struct SubmitButtonProps {
    label: String,
}

#[component]
pub fn SubmitButton(props: SubmitButtonProps) -> View {
    view! {
        button(class="submit-btn", r#type="submit") {
            (props.label)
        }
    }
}
