use sycamore::prelude::*;

use crate::components::common::ServerContext;

use super::member_item::MemberItem;

#[component(inline_props)]
pub fn MembersPanel() -> View {
    let server_context = use_context::<ServerContext>();

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
