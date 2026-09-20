use gilvave_core::{
    dto::{
        channel::ChannelView,
        message::MessageView,
        server::{MemberView, Server, ServerSmallPart},
    },
    ids::ServerId,
};
use sycamore::prelude::*;

#[derive(Clone)]
pub struct ChannelContext {
    pub current: Signal<Option<ChannelView>>,
    pub text: Signal<Vec<ChannelView>>,
    pub voice: Signal<Vec<ChannelView>>,
    pub messages: Signal<Vec<MessageView>>,
}

#[derive(Clone)]
pub struct ServerContext {
    pub current: Signal<Option<Server>>,
    pub list: Signal<Vec<ServerSmallPart>>,
    pub members: Signal<Vec<MemberView>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ModalView {
    Home,
    Create,
    Join,
}

#[derive(Clone)]
pub struct CreateServerContext {
    pub is_modal_open: Signal<bool>,
    pub modal_view: Signal<ModalView>,
    pub server_name: Signal<String>,
    pub is_public: Signal<bool>,
    pub public_servers: Signal<Vec<Server>>,
    pub expanded_id: Signal<Option<ServerId>>,
    pub from_dashboard: Signal<bool>,
}

#[derive(Clone)]
pub struct UserProfileContext {
    pub username: Signal<String>,
    pub avatar: Signal<String>,
    pub banner: Signal<String>,
    pub bio: Signal<String>,
    pub is_muted: Signal<bool>,
    pub is_deafened: Signal<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum HomeTab {
    #[default]
    Chats,
    Dashboard,
}

#[derive(Clone)]
pub struct UiModalContext {
    pub is_server_settings_open: Signal<bool>,
    pub is_create_channel_open: Signal<bool>,
    pub create_channel_type: Signal<gilvave_core::dto::channel::ChannelType>,
    pub is_profile_settings_open: Signal<bool>,
    pub selected_dm_name: Signal<Option<String>>,
    pub home_tab: Signal<HomeTab>,
}

