use sycamore::prelude::*;

use crate::components::common::{CreateServerContext, classes};

use super::server_card::server_card;

#[component(inline_props)]
pub(super) fn JoinServerModal(
    is_visible: ReadSignal<bool>,
    on_back: impl Fn(web_sys::MouseEvent) + 'static,
    on_close: impl Fn(web_sys::MouseEvent) + 'static,
) -> View {
    let context = use_context::<CreateServerContext>();

    view! {
        div(
            class=classes(vec![
                "server-modal-overlay".into(),
                "large".into(),
                ("hidden", { is_visible.map(|visible| !visible) }.into()).into(),
            ]),
            on:click=on_close,
        ) {
            div(class="server-modal join-modal", on:click=move |event: web_sys::MouseEvent| event.stop_propagation()) {
                div(class="server-modal-header") {
                    span { "Присоединиться к серверу" }
                    div(class="join-modal-search") {
                        input(r#type="text", placeholder="Поиск серверов...")
                    }
                }
                div(class="join-modal-body") {
                    div(class="join-modal-grid") {
                        Indexed(
                            list=context.public_servers,
                            view=move |server| server_card(server, context.expanded_id),
                        )
                    }
                }
                div(class="join-modal-footer") {
                    button(class="back-btn", on:click=on_back) { "← Назад" }
                }
            }
        }
    }
}
