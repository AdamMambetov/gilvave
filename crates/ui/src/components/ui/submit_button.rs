use sycamore::prelude::*;

#[component(inline_props)]
pub fn SubmitButton(
    #[prop(attributes(html, button))] attributes: Attributes,
    children: Children,
) -> View {
    view! {
        button(class="submit-btn", r#type="submit", ..attributes) {
            (children)
        }
    }
}
