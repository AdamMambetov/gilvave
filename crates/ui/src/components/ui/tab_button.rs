use sycamore::prelude::*;

use crate::components::common::class_name;

#[component(inline_props)]
pub fn TabButton(
    #[prop(attributes(html, button))] attributes: Attributes,
    children: Children,
    is_active: MaybeDyn<bool>,
) -> View {
    view! {
        button(
            class=class_name("tab-btn", &is_active, "active", ""),
            ..attributes,
        ) {
            (children)
        }
    }
}
