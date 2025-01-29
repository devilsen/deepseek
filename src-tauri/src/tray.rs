use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let open_i = MenuItem::with_id(app, "open", "打开 DeepSeek 窗口", true, None::<&str>)?;
    let setting_i = MenuItem::with_id(app, "setting", "设置", true, None::<&str>)?;
    let check_update_i: MenuItem<R> =
        MenuItem::with_id(app, "check_update", "检查更新", true, None::<&str>)?;

    let log_out_i: MenuItem<R> = MenuItem::with_id(app, "logout", "注销", true, None::<&str>)?;
    let quit_i: MenuItem<R> = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // 分割线
    let menu = Menu::with_items(
        app,
        &[&open_i, &setting_i, &check_update_i, &log_out_i, &quit_i],
    )?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "open" => {
                let window = app.get_webview_window("main").unwrap();
                let _ = window.show();
            }
            "quit" => {
                app.exit(0);
            }
            "setting" => {
                // let window = app.get_webview_window("main").unwrap();
                // let _ = window.hide();
                println!("setting");
            }
            "check_update" => {
                println!("check_update");
            }
            "logout" => {
                println!("logout");
            }
            // Add more events here
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);

    Ok(())
}
