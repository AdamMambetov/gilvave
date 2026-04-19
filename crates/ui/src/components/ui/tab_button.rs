use sycamore::prelude::*;

use crate::components::common::classes;

#[component(inline_props)]
pub fn TabButton(
    #[prop(attributes(html, button))] attributes: Attributes,
    children: Children,
    is_active: MaybeDyn<bool>,
) -> View {
    view! {
        button(
            class=classes(vec![
                "tab-btn".into(),
                ("active", is_active.clone()).into(),
            ]),
            ..attributes,
        ) {
            (children)
        }
    }
}
