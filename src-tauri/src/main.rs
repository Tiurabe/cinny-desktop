#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
mod menu;

use tauri::{utils::config::AppUrl, WindowUrl};
#[cfg(any(target_os = "windows", target_os = "linux"))]
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent,
};

#[cfg(any(target_os = "windows", target_os = "linux"))]
const MAIN_WINDOW_LABEL: &str = "main";
#[cfg(any(target_os = "windows", target_os = "linux"))]
const TRAY_SHOW_ID: &str = "show";
#[cfg(any(target_os = "windows", target_os = "linux"))]
const TRAY_QUIT_ID: &str = "quit";

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn create_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(TRAY_SHOW_ID.to_owned(), "Show Cinny"))
        .add_item(CustomMenuItem::new(TRAY_QUIT_ID.to_owned(), "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } | SystemTrayEvent::DoubleClick { .. } => {
            show_main_window(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            TRAY_SHOW_ID => show_main_window(app),
            TRAY_QUIT_ID => app.exit(0),
            _ => {}
        },
        _ => {}
    }
}

fn main() {
    let port = 44548;

    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());
    let builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    let builder = builder.menu(menu::menu());

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    let builder = builder
        .system_tray(create_tray())
        .on_system_tray_event(handle_tray_event)
        .on_window_event(|event| {
            if event.window().label() != MAIN_WINDOW_LABEL {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                api.prevent_close();
                let _ = event.window().hide();
            }
        });

    builder
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(context)
        .expect("error while building tauri application")
}
