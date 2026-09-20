use futures_util::StreamExt;
use gilvave_core::dto::{command::CommandArgs, message::MessageView};
use sycamore::{futures::spawn_local_scoped, prelude::*, web::queue_microtask};
use tauri_sys::event::listen;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, SubmitEvent};

use super::message_item::MessageItem;
use crate::{
    components::common::{ChannelContext, ServerContext, UserProfileContext},
    utils::{get_local_offset, invoke_command, to_local_datetime},
};

#[derive(Clone, PartialEq)]
struct EnrichedMessage {
    message: MessageView,
    avatar_url: String,
    is_first: bool,
    is_last: bool,
    date_divider: Option<String>,
}

fn format_date_divider(date: time::Date) -> String {
    let month_str = match date.month() {
        time::Month::January => "января",
        time::Month::February => "февраля",
        time::Month::March => "марта",
        time::Month::April => "апреля",
        time::Month::May => "мая",
        time::Month::June => "июня",
        time::Month::July => "июля",
        time::Month::August => "августа",
        time::Month::September => "сентября",
        time::Month::October => "октября",
        time::Month::November => "ноября",
        time::Month::December => "декабря",
    };

    let weekday_str = match date.weekday() {
        time::Weekday::Monday => "понедельник",
        time::Weekday::Tuesday => "вторник",
        time::Weekday::Wednesday => "среда",
        time::Weekday::Thursday => "четверг",
        time::Weekday::Friday => "пятница",
        time::Weekday::Saturday => "суббота",
        time::Weekday::Sunday => "воскресенье",
    };

    let now = to_local_datetime(time::OffsetDateTime::now_utc());
    let today = now.date();
    let yesterday = (now - time::Duration::days(1)).date();

    if date == today {
        format!("Сегодня, {} {}", date.day(), month_str)
    } else if date == yesterday {
        format!("Вчера, {} {}", date.day(), month_str)
    } else if date.year() == today.year() {
        format!("{} {}, {}", date.day(), month_str, weekday_str)
    } else {
        format!("{} {} {} г.", date.day(), month_str, date.year())
    }
}

#[component()]
pub fn MessagesArea() -> View {
    let channel_context = use_context::<ChannelContext>();
    let server_context = use_context::<ServerContext>();
    let user_profile = use_context::<UserProfileContext>();
    let channel_name = create_memo(move || match channel_context.current.get_clone() {
        Some(channel) => channel.name,
        None => "<UNKNOWN>".into(),
    });

    let message_text = create_signal(String::new());
    let container = create_node_ref();
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let msg = message_text.get_clone().trim().to_string();
        let channel = channel_context.current.get_clone();
        if msg.is_empty() || channel.is_none() {
            return;
        }

        spawn_local_scoped(async move {
            let args = CommandArgs::MessageCreate {
                channel_id: channel.unwrap().id,
                content: msg,
            }
            .to_json();
            invoke_command(args).await;
        });
        message_text.set(String::new());
    };
    let on_scroll = move |event: Event| {
        let el: HtmlElement = match event.current_target() {
            Some(t) => t.unchecked_into(),
            None => return,
        };

        const SCROLL_THRESHOLD: i32 = 1;

        let top = el.scroll_top();
        let sh = el.scroll_height();
        let ch = el.client_height();

        let is_top = top <= 0;
        let is_bottom = (sh - ch - top).abs() <= SCROLL_THRESHOLD;

        if is_top {
            console_log!("top message");
            spawn_local_scoped(async move {
                let context = use_context::<ChannelContext>();
                let args = CommandArgs::ChannelHistoryBefore {
                    channel_id: context.current.get_clone().unwrap().id,
                    timestamp: channel_context
                        .messages
                        .get_clone()
                        .first()
                        .unwrap()
                        .created_at,
                }
                .to_json();
                invoke_command(args).await;
            });
        }
        if is_bottom {
            console_log!("bottom message");
            spawn_local_scoped(async move {
                let context = use_context::<ChannelContext>();
                let args = CommandArgs::ChannelHistoryAfter {
                    channel_id: context.current.get_clone().unwrap().id,
                    timestamp: channel_context
                        .messages
                        .get_clone()
                        .last()
                        .unwrap()
                        .created_at,
                }
                .to_json();
                invoke_command(args).await;
            });
        }
    };

    create_effect(move || {
        channel_context.current.track();
        channel_context.messages.set(vec![]);
    });

    spawn_local_scoped(async move {
        let mut events = listen::<MessageView>("message_new").await.unwrap();
        while let Some(event) = events.next().await {
            let el = container.get().unchecked_into::<HtmlElement>();
            let old_scroll_top = el.scroll_top();
            let old_scroll_height = el.scroll_height();
            let old_client_height = el.client_height();

            const SCROLL_THRESHOLD: i32 = 1;
            let is_bottom =
                (old_scroll_height - old_client_height - old_scroll_top).abs() <= SCROLL_THRESHOLD;

            channel_context
                .messages
                .update(|list| list.push(event.payload));

            queue_microtask(move || {
                if is_bottom {
                    let delta = el.scroll_height() - old_scroll_height;
                    el.set_scroll_top(old_scroll_top + delta);
                }
            });
        }
    });
    spawn_local_scoped(async move {
        let mut events = listen::<Vec<MessageView>>("channel_history_before")
            .await
            .unwrap();
        while let Some(event) = events.next().await {
            let el = container.get().unchecked_into::<HtmlElement>();
            let old_scroll_top = el.scroll_top();
            let old_scroll_height = el.scroll_height();

            channel_context.messages.update(|list| {
                for msg in event.payload.into_iter() {
                    list.insert(0, msg);
                }
            });

            // Ждём, пока Sycamore реально вольёт узлы в DOM.
            // queue_microtask достаточно, т.к. эффекты Sycamore выполняются
            // в микротасках. Для надёжности можно использовать RAF.
            queue_microtask(move || {
                let delta = el.scroll_height() - old_scroll_height;
                el.set_scroll_top(old_scroll_top + delta);
            });
        }
    });
    spawn_local_scoped(async move {
        let mut events = listen::<Vec<MessageView>>("channel_history_after")
            .await
            .unwrap();
        while let Some(mut event) = events.next().await {
            channel_context
                .messages
                .update(|list| list.append(event.payload.as_mut()));
        }
    });

    let on_back = move |_| {
        channel_context.current.set(None);
    };

    let enriched_messages = create_memo(move || {
        let msgs = channel_context.messages.get_clone();
        let members = server_context.members.get_clone();
        let my_username = user_profile.username.get_clone();
        let my_avatar = user_profile.avatar.get_clone();
        let len = msgs.len();

        let local_offset = get_local_offset();
        let local_msgs: Vec<(time::OffsetDateTime, MessageView)> = msgs
            .into_iter()
            .map(|m| {
                let local_dt = m.created_at.to_offset(local_offset);
                (local_dt, m)
            })
            .collect();

        local_msgs
            .iter()
            .enumerate()
            .map(|(i, (m_local_dt, m))| {
                let m_date = m_local_dt.date();
                let is_new_day = if i == 0 {
                    true
                } else {
                    local_msgs[i - 1].0.date() != m_date
                };
                let date_divider = if is_new_day {
                    Some(format_date_divider(m_date))
                } else {
                    None
                };

                let is_first = if i == 0 {
                    true
                } else {
                    let (prev_dt, prev_m) = &local_msgs[i - 1];
                    prev_m.author_name != m.author_name
                        || (m.author_id.is_some() && prev_m.author_id != m.author_id)
                        || prev_dt.date() != m_date
                        || (*m_local_dt - *prev_dt) > time::Duration::minutes(5)
                };

                let is_last = if i + 1 == len {
                    true
                } else {
                    let (next_dt, next_m) = &local_msgs[i + 1];
                    next_m.author_name != m.author_name
                        || (m.author_id.is_some() && next_m.author_id != m.author_id)
                        || next_dt.date() != m_date
                        || (*next_dt - *m_local_dt) > time::Duration::minutes(5)
                };

                let avatar = if !my_avatar.is_empty() && m.author_name == my_username {
                    my_avatar.clone()
                } else {
                    members
                        .iter()
                        .find(|mem| {
                            mem.username == m.author_name
                                || (m.author_id.is_some() && Some(mem.user_id) == m.author_id)
                        })
                        .map(|mem| mem.avatar.clone())
                        .unwrap_or_else(|| {
                            if m.author_name == my_username {
                                my_avatar.clone()
                            } else {
                                String::new()
                            }
                        })
                };

                EnrichedMessage {
                    message: m.clone(),
                    avatar_url: avatar,
                    is_first,
                    is_last,
                    date_divider,
                }
            })
            .collect::<Vec<_>>()
    });

    view! {
        div(class="messages-area") {
            div(class="chat-header") {
                button(class="chat-back-btn", on:click=on_back) {
                    svg(
                        xmlns="http://www.w3.org/2000/svg",
                        width="18",
                        height="18",
                        viewBox="0 0 24 24",
                        fill="none",
                        stroke="currentColor",
                        stroke-width="2.5",
                        stroke-linecap="round",
                        stroke-linejoin="round",
                    ) {
                        polyline(points="15 18 9 12 15 6")
                    }
                    span { "Каналы" }
                }
                div(class="chat-header-title") {
                    span(class="channel-hash") { "#" }
                    span(class="channel-name") { (channel_name) }
                }
            }

            div(class="messages-list", r#ref=container, on:scroll=on_scroll) {
                WelcomeMessage(channel_name=channel_name)

                Keyed(
                    list=enriched_messages,
                    key=|em| (em.message.id, em.is_first, em.is_last, em.date_divider.clone(), em.avatar_url.clone()),
                    view=|em| {
                        let date_view = if let Some(ref date_str) = em.date_divider {
                            let d = date_str.clone();
                            view! {
                                div(class="chat-date-divider") {
                                    span(class="date-badge") { (d) }
                                }
                            }
                        } else {
                            view! {}
                        };

                        view! {
                            (date_view)
                            MessageItem(
                                message_view=em.message,
                                avatar_url=em.avatar_url,
                                is_first_in_group=em.is_first,
                                is_last_in_group=em.is_last,
                            )
                        }
                    },
                )
            }

            form(class="input-area", on:submit=on_submit) {
                input(
                    class="chat-input",
                    placeholder="Напишите сообщение...",
                    bind:value=message_text,
                )
                button(class="send-btn", r#type="submit", title="Отправить") {
                    svg(
                        xmlns="http://www.w3.org/2000/svg",
                        width="18",
                        height="18",
                        viewBox="0 0 24 24",
                        fill="none",
                        stroke="currentColor",
                        stroke-width="2",
                        stroke-linecap="round",
                        stroke-linejoin="round",
                    ) {
                        line(x1="22", y1="2", x2="11", y2="13")
                        polygon(points="22 2 15 22 11 13 2 9 22 2")
                    }
                }
            }
        }
    }
}

#[component(inline_props)]
fn WelcomeMessage(channel_name: ReadSignal<String>) -> View {
    view! {
        div(class="welcome-message") {
            p { (format!("Добро пожаловать на канал {channel_name}!")) }
        }
    }
}
