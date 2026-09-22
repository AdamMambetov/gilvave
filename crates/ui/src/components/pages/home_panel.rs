use gilvave_core::dto::{
    channel::ChannelView,
    command::{CommandArgs, CommandResponse, CommandResult},
    message::MessageView,
    server::{MemberView, Server, ServerSmallPart},
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::{
        common::{
            ChannelContext, ScreenWrapper, ServerContext, UiModalContext, UserProfileContext,
            classes,
        },
        features::{
            channels::{channel_panel::ChannelPanel, create_channel_modal::CreateChannelModal},
            chat::messages_area::MessagesArea,
            home::{home_dashboard::HomeDashboard, home_nav_panel::HomeNavPanel},
            members::members_panel::MembersPanel,
            profile::profile_settings_modal::ProfileSettingsModal,
            servers::{server_settings_modal::ServerSettingsModal, server_sidebar::ServerSidebar},
        },
    },
    utils::invoke_command,
};

#[component]
pub fn HomePanel() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let is_home_screen: MaybeDyn<bool> = (move || screen_wrapper.is_home()).into();

    let user_profile = UserProfileContext {
        username: create_signal(String::default()),
        avatar: create_signal(String::default()),
        banner: create_signal(String::default()),
        bio: create_signal(String::default()),
        is_muted: create_signal(false),
        is_deafened: create_signal(false),
    };
    provide_context(user_profile.clone());

    let ui_modal_context = UiModalContext {
        is_server_settings_open: create_signal(false),
        is_create_channel_open: create_signal(false),
        create_channel_type: create_signal(gilvave_core::dto::channel::ChannelType::TEXT),
        is_profile_settings_open: create_signal(false),
        selected_dm_name: create_signal(None),
        home_tab: create_signal(crate::components::common::HomeTab::Chats),
    };
    provide_context(ui_modal_context.clone());

    let server_context = ServerContext {
        current: create_signal::<Option<Server>>(None),
        list: create_signal::<Vec<ServerSmallPart>>(vec![]),
        members: create_signal::<Vec<MemberView>>(vec![]),
    };
    provide_context(server_context.clone());

    let channel_context = ChannelContext {
        text: create_signal::<Vec<ChannelView>>(vec![]),
        voice: create_signal::<Vec<ChannelView>>(vec![]),
        current: create_signal::<Option<ChannelView>>(None),
        messages: create_signal::<Vec<MessageView>>(vec![]),
    };
    provide_context(channel_context.clone());

    let ws_started = create_signal(false);
    let u_prof_for_effect = user_profile.clone();
    create_effect(move || {
        if screen_wrapper.is_home() {
            let u_prof = u_prof_for_effect.clone();
            spawn_local_scoped(async move {
                let args = CommandArgs::GetProfile.to_json();
                let res = invoke_command(args).await;
                if let CommandResult::Ok(CommandResponse::GetProfile(user)) = res {
                    u_prof.username.set(user.username);
                    u_prof.avatar.set(user.avatar);
                }
            });

            if !ws_started.get() {
                ws_started.set(true);
                spawn_local_scoped(async move {
                    invoke_command(CommandArgs::ListenWebSocket.to_json()).await;
                });
            }
            spawn_local_scoped(async move {
                let res = invoke_command(CommandArgs::GetUserServers.to_json()).await;
                server_context.list.set(
                    if let CommandResult::Ok(CommandResponse::GetUserServers(servers)) = res {
                        servers
                    } else {
                        vec![]
                    },
                );
            });
        }
    });

    let handle_home_click = move |_| {
        let ch_context = use_context::<ChannelContext>();
        let active_channel = ch_context.current.get_clone();
        ch_context.current.set(None);
        if let Some(channel) = active_channel {
            spawn_local_scoped(async move {
                let args = CommandArgs::LeftChannel {
                    channel_id: channel.id,
                }
                .to_json();
                invoke_command(args).await;
            });
        }
        let context = use_context::<ServerContext>();
        context.current.set(None);
        let m_ctx = use_context::<UiModalContext>();
        m_ctx.selected_dm_name.set(None);
    };

    let is_home_active = create_memo(move || server_context.current.get_clone().is_none());
    let home_pill_class = move || if is_home_active.get() { "home-pill active" } else { "home-pill" };
    let home_btn_class = move || if is_home_active.get() { "home-avatar-btn active" } else { "home-avatar-btn" };

    let discord_content_class = create_memo(move || {
        if server_context.current.with(|c| c.is_some()) {
            "discord-content in-server"
        } else {
            match ui_modal_context.home_tab.get() {
                crate::components::common::HomeTab::Chats => "discord-content in-home show-chats",
                crate::components::common::HomeTab::Dashboard => "discord-content in-home show-dashboard",
            }
        }
    });

    let is_server_settings_open = ui_modal_context.is_server_settings_open;
    let is_create_channel_open = ui_modal_context.is_create_channel_open;
    let is_profile_settings_open = ui_modal_context.is_profile_settings_open;

    view! {
        div(
            class=classes(vec![
                "discord-container".into(),
                "home-panel-container".into(),
                ("active", is_home_screen.clone()).into(),
            ]),
        ) {
            div(class="discord-sidebar") {
                div(class="home-button-wrapper") {
                    div(class=home_pill_class)
                    button(
                        class=home_btn_class,
                        on:click=handle_home_click,
                        title="Главная (Личные сообщения и друзья)",
                    ) {
                        div(class="home-avatar-inner") {
                            (if !user_profile.avatar.get_clone().is_empty() {
                                let av = user_profile.avatar.get_clone();
                                view! { img(src=av, alt="") }
                            } else {
                                let initial = user_profile.username.get_clone().chars().next().unwrap_or('?').to_uppercase().to_string();
                                view! { span { (initial) } }
                            })
                        }
                        div(class="home-badge") {
                            "🏠"
                        }
                    }
                }

                div(class="separator")
                ServerSidebar()
            }

            div(class="discord-main") {
                div(class="discord-header") {
                    (header_server_view(server_context.current.get_clone(), ui_modal_context.clone()))

                    div(class="search-bar") {
                        span { "🔍 Поиск" }
                    }
                }

                div(class=discord_content_class) {
                    (if server_context.current.get_clone().is_some() {
                        view! {
                            ChannelPanel()

                            (if channel_context.current.get_clone().is_some() {
                                view! { MessagesArea() }
                            } else {
                                view! {
                                    div(class="no-channel-selected") {
                                        div(class="no-channel-content") {
                                            span(class="no-channel-icon") { "#" }
                                            h3 { "Выберите канал" }
                                            p { "Выберите текстовый или голосовой канал в списке слева, чтобы начать общение." }
                                        }
                                    }
                                }
                            })
                        }
                    } else {
                        view! {
                            HomeNavPanel()
                            HomeDashboard()
                        }
                    })
                }
            }

            (if server_context.current.get_clone().is_some() {
                MembersPanel()
            } else {
                view!{}
            })

            (if is_server_settings_open.get() {
                view! { ServerSettingsModal() }
            } else {
                view! {}
            })
            (if is_create_channel_open.get() {
                view! { CreateChannelModal() }
            } else {
                view! {}
            })
            (if is_profile_settings_open.get() {
                view! { ProfileSettingsModal() }
            } else {
                view! {}
            })
        }
    }
}

fn header_server_view(server_opt: Option<Server>, modal_context: UiModalContext) -> View {
    if let Some(server) = server_opt {
        let cover_view = if !server.cover.is_empty() {
            let c = server.cover.clone();
            view! {
                div(class="header-server-cover") {
                    img(src=c, alt="")
                }
            }
        } else {
            view! {}
        };

        let icon_view = if !server.icon_url.is_empty() {
            let i = server.icon_url.clone();
            view! { img(src=i, alt="") }
        } else {
            let initial = server
                .name
                .chars()
                .next()
                .unwrap_or('?')
                .to_uppercase()
                .to_string();
            view! { span { (initial) } }
        };

        let members_str = format!("{} участников", server.members_count);
        let s_name = server.name.clone();

        view! {
            div(class="header-server-info") {
                (cover_view)
                div(class="header-server-icon") {
                    (icon_view)
                }
                div(class="header-server-text") {
                    span(class="header-server-name") { (s_name) }
                    span(class="header-server-members") { (members_str) }
                }
                button(
                    class="header-settings-btn",
                    on:click=move |_| modal_context.is_server_settings_open.set(true),
                    title="Настройки сервера",
                ) {
                    "⚙️"
                }
            }
        }
    } else {
        view! {
            div(class="header-server-info") {
                div(class="header-server-icon home") {
                    span { "🏠" }
                }
                div(class="header-server-text") {
                    span(class="header-server-name") { "Личные сообщения" }
                }
            }
        }
    }
}
