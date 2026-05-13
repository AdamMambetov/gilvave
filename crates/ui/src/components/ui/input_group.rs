use sycamore::prelude::*;

use crate::components::common::classes;

#[component(inline_props)]
pub fn InputGroup(
    #[prop(attributes(html, input))] attributes: Attributes,
    label: Option<&'static str>,
    is_error: MaybeDyn<bool>,
    error_message: Option<&'static str>,
) -> View {
    view! {
        div(
            class=classes(vec![
                "input-group".into(),
                ("error", is_error.clone()).into()
            ]),
        ) {
            label { (label.unwrap_or("")) }
            input(..attributes)
            div(class="error-message") { (error_message.unwrap_or("")) }
        }
    }
}
