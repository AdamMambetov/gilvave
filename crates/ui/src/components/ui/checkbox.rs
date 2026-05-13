use sycamore::prelude::*;

#[component(inline_props)]
pub fn Checkbox(
    #[prop(attributes(html, input))] attributes: Attributes,
    children: Children,
) -> View {
    view! {
        label(class="checkbox") {
            input(
                r#type="checkbox",
                ..attributes,
            )
            (children)
        }
    }
}
