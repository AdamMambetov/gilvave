#[cfg(not(target_arch = "wasm32"))]
use keyring::Entry;

#[cfg(not(target_arch = "wasm32"))]
fn get_keyring(key: &str) -> Entry {
    Entry::new("gilvave-app", key).unwrap()
}

pub fn get_access_token() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("access_token").ok().flatten())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        get_keyring("access_token")
            .get_password()
            .unwrap_or_default()
    }
}

pub fn get_refresh_token() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("refresh_token").ok().flatten())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        get_keyring("refresh_token")
            .get_password()
            .unwrap_or_default()
    }
}

pub fn set_access_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("access_token", token);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        get_keyring("access_token").set_password(token).ok();
    }
}

pub fn set_refresh_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("refresh_token", token);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        get_keyring("refresh_token").set_password(token).ok();
    }
}

