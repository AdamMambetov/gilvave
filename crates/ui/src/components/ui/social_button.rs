use sycamore::prelude::*;

#[component(inline_props)]
pub fn SocialButton(
    #[prop(attributes(html, button))] attributes: Attributes,
    children: Children,
) -> View {
    view! {
        button(class="social-btn", r#type="button", ..attributes) {
            (children)
        }
    }
}
