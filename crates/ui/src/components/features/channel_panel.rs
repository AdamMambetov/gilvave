use sycamore::prelude::*;

use crate::components::pages::home_panel::ChannelContext;

use super::channel_item::ChannelItem;

#[component(inline_props)]
pub fn ChannelPanel() -> View {
    let context = use_context::<ChannelContext>();
    let text_channel_list = context.text;
    let voice_channel_list = context.voice;

    view! {
        div(class="channel-panel") {
            div(class="channel-list") {
                div(class="channel-header") { "Текстовые" }
                Indexed(
                    list=text_channel_list,
                    view=|channel| { view! {
                        ChannelItem(channel_view=channel.clone())
                    }},
                )
            }

            div(class="channel-list") {
                div(class="channel-header") { "Голосовые" }
                Indexed(
                    list=voice_channel_list,
                    view=|channel| { view! { div(class="channel-item") { (channel.name) } } },
                )
            }
        }
    }
}
