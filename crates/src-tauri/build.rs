#[cfg(target_os = "linux")]
fn set_linux_env() {
    println!("cargo:rustc-env=WEBKIT_DISABLE_DMABUF_RENDERER=1");
    println!("cargo:rustc-env=WEBKIT_DISABLE_COMPOSITING_MODE=1");
    println!("cargo:rustc-env=GDK_BACKEND=wayland");
}

#[cfg(not(target_os = "linux"))]
fn set_linux_env() {}

fn main() {
    set_linux_env();

    tauri_build::build()
}
