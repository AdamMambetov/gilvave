use sycamore::prelude::*;

use crate::components::common::MemberContext;

use super::member_item::MemberItem;

#[component(inline_props)]
pub fn MembersPanel() -> View {
    let member_list = use_context::<MemberContext>().0;
    view! {
        div(class="discord-members-panel") {
            div(class="panel-section") {
                h3 { "Онлайн" }
                Indexed(
                    list=member_list,
                    view=|member| { view! { MemberItem(member_view=member) } },
                )
            }

            div(class="panel-section") {
                h3 { "Офлайн" }
            }
        }
    }
}
