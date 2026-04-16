use sycamore::prelude::*;

use crate::components::common::class_name;

#[derive(Props)]
pub struct InputGroupProps {
    label: String,
    r#type: String,
    placeholder: String,
    bind_value: Signal<String>,
    error_condition: Box<dyn Fn() -> bool>,
    error_message: String,
    // on_input: Box<dyn Fn(Event)>,
}

#[component]
pub fn InputGroup(props: InputGroupProps) -> View {
    view! {
        div(class=class_name("input-group", props.error_condition.as_ref(), "error", "")) {
            label { (props.label) }
            input(
                r#type=props.r#type,
                placeholder=props.placeholder,
                bind:value=props.bind_value,
                // on:input=props.on_input,
            )
            div(class="error-message") { (props.error_message) }
        }
    }
}
