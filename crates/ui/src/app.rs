use sycamore::prelude::*;

use crate::components::templates::auth_form::AuthForm;

#[component]
pub fn App() -> View {
    view! {
        main(class="container") {
            AuthForm()
        }
    }
}
