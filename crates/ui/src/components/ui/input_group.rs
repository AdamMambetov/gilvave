use sycamore::prelude::*;

use crate::components::common::{class_name, classes};

#[component(inline_props)]
pub fn InputGroup(
    #[prop(attributes(html, input))] attributes: Attributes,
    label: Option<&'static str>,
    is_error: MaybeDyn<bool>,
    error_message: Option<&'static str>,
) -> View {
    view! {
        // div(class=class_name("input-group", &is_error, "error", "")) {
        div(class=classes(vec![
            "input-group".into(),
            ("error", is_error.clone()).into()
        ])) {
        // div(class=|| {
        //     let mut c = vec!["input-group"];
        //     if is_error.get() { c.push("error"); }
        //     c.join(" ")
        // }) {
            label { (label.unwrap_or("")) }
            input(..attributes)
            div(class="error-message") { (error_message.unwrap_or("")) }
        }
    }
}
