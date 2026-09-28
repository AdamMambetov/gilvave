use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{components::common::ServerContext, http::api::Api};

use super::member_item::MemberItem;

#[component()]
pub fn MembersPanel() -> View {
    let server_context = use_context::<ServerContext>();

    let current_server_id = create_memo(move || {
        server_context
            .current
            .with(|s| s.as_ref().map(|srv| srv.id))
    });

    create_effect(move || {
        if let Some(server_id) = current_server_id.get() {
            spawn_local_scoped(async move {
                if let Ok(members) = Api::get_members(server_id).await {
                    server_context.members.set(members);
                }
            })
        } else {
            server_context.members.set(vec![]);
        }
    });

    view! {
        div(class="discord-members-panel") {
            div(class="panel-section") {
                h3 { "Онлайн" }
                Indexed(
                    list=server_context.members,
                    view=|member| { view! { MemberItem(member_view=member) } },
                )
            }

            div(class="panel-section") {
                h3 { "Офлайн" }
            }
        }
    }
}
