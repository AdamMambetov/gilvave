use sycamore::prelude::*;

#[derive(Props)]
pub struct CheckboxProps {
    label: String,
    bind_checked: Signal<bool>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> View {
    view! {
        label(class="checkbox") {
            input(
                r#type="checkbox",
                bind:checked=props.bind_checked,
                // on:input=toggle_remember,
            )
            (props.label)
        }
    }
}
