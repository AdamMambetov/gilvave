use std::rc::Rc;

use gilvave_core::dto::{
    command::{CommandArgs, CommandResponse, CommandResult},
    server::ServerView,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::common::{ActiveScreen, ChannelContext, MemberContext, ScreenWrapper},
    utils::invoke_command,
};

#[component(inline_props)]
pub fn ServerSidebar(
    server_list: Signal<Vec<ServerView>>,
    on_create_server: impl Fn() + 'static,
) -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let show_modal = create_signal(false);
    let on_create_server_rc: Rc<dyn Fn()> = Rc::new(on_create_server);

    let handle_home_click = move |_| {
        spawn_local_scoped(async {
            let context = use_context::<ChannelContext>();
            if context.current.get().is_some() {
                let args = CommandArgs::LeftChannel {
                    channel_id: context.current.get().unwrap(),
                }
                .to_json();
                context.current.set(None);
                invoke_command(args).await;
            }
        });
        // TODO: сделать понятнее обновление списка серверов
        //       при нажатии на home
        screen_wrapper.set(ActiveScreen::Login);
        screen_wrapper.set(ActiveScreen::Home);
    };

    let on_plus_click = move |_| {
        show_modal.set(true);
    };

    view! {
        div(class="discord-sidebar") {
            div(class="server-icon", on:click=handle_home_click) { "🏠" }
            div(class="separator") {}
            Indexed(
                list=server_list,
                view=|server| {
                    let server_name = server.name;
                    view! {
                        div(
                            class="server-icon",
                            on:click=move |_| on_click_server(server.id),
                        ) { (server_name) }
                    }
                },
            )
            div(
                class="server-icon new",
                on:click=on_plus_click,
            ) { "+" }
        }

        (if show_modal.get() {
            let sm = show_modal.clone();
            let cb = on_create_server_rc.clone();
            view! {
                div(class="server-modal-overlay", on:click=move |_| sm.set(false)) {
                    div(class="server-modal", on:click=move |e: web_sys::MouseEvent| e.stop_propagation()) {
                        div(class="server-modal-header") {
                            span { "Серверы" }
                        }
                        div(class="server-modal-body") {
                            div(class="server-modal-card create", on:click=move |_| {
                                show_modal.set(false);
                                cb();
                            }) {
                                div(class="server-modal-icon create") {
                                    svg(xmlns="http://www.w3.org/2000/svg", width="32", height="32", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round") {
                                        line(x1="12", y1="5", x2="12", y2="19")
                                        line(x1="5", y1="12", x2="19", y2="12")
                                    }
                                }
                                span(class="server-modal-label") { "Создать сервер" }
                            }
                            div(class="server-modal-card join") {
                                div(class="server-modal-icon join") {
                                    svg(xmlns="http://www.w3.org/2000/svg", width="32", height="32", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round") {
                                        path(d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4")
                                        polyline(points="10 17 15 12 10 7")
                                        line(x1="15", y1="12", x2="3", y2="12")
                                    }
                                }
                                span(class="server-modal-label") { "Присоединиться" }
                            }
                        }
                    }
                }
            }
        } else {
            view! {}
        })
    }
}

fn on_click_server(server_id: gilvave_core::ids::ServerId) {
    spawn_local_scoped(async move {
        let context = use_context::<MemberContext>();
        let args = CommandArgs::GetMembers {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.0.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetMembers(members)) = res {
            context.0.set(members);
        }
    });
    spawn_local_scoped(async move {
        let context = use_context::<ChannelContext>();
        if context.current.get().is_some() {
            let args = CommandArgs::LeftChannel {
                channel_id: context.current.get().unwrap(),
            }
            .to_json();
            context.current.set(None);
            invoke_command(args).await;
        }

        let args = CommandArgs::GetServerChannels {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.text.set(vec![]);
        context.voice.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetServerChannels(channels)) = res {
            for channel in channels {
                match channel.r#type {
                    gilvave_core::dto::channel::ChannelType::TEXT => {
                        context.text.update(|list| list.push(channel))
                    }
                    gilvave_core::dto::channel::ChannelType::VOICE => {
                        context.voice.update(|list| list.push(channel))
                    }
                }
            }
        }
    })
}
