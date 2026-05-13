use keyring::Entry;

fn get_keyring(key: &str) -> Entry {
    Entry::new("gilvave-app", key).unwrap()
}

pub fn get_access_token() -> String {
    get_keyring("access_token").get_password().unwrap()
}

pub fn get_refresh_token() -> String {
    get_keyring("refresh_token").get_password().unwrap()
}

pub fn set_access_token(token: &str) {
    get_keyring("access_token").set_password(token).ok();
}

pub fn set_refresh_token(token: &str) {
    get_keyring("refresh_token").set_password(token).ok();
}
