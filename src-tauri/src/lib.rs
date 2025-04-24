use tauri::Manager;

mod command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().setup(|app| {
        #[cfg(dev)]
        app.get_webview_window("main",).unwrap().open_devtools();

        Ok((),)
    },);

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let _ = app
                    .get_webview_window("main",)
                    .expect("no main window",)
                    .set_focus();
            },),)
            .plugin(tauri_plugin_updater::Builder::new().build(),);
    }

    builder
        .plugin(tauri_plugin_dialog::init(),)
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ),)
                .level(log::LevelFilter::Debug,)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal,)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll,)
                .build(),
        )
        .plugin(tauri_plugin_fs::init(),)
        .plugin(tauri_plugin_os::init(),)
        .plugin(tauri_plugin_process::init(),)
        .plugin(tauri_plugin_shell::init(),)
        .plugin(tauri_plugin_system_info::init(),)
        .plugin(prevent_default(),)
        .run(tauri::generate_context!(),)
        .expect("error while running tauri application",);
}

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry,> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
        .with_flags(
            Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD | Flags::CONTEXT_MENU,),
        )
        .build()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry,> {
    tauri_plugin_prevent_default::init()
}
