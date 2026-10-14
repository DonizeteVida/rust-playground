use hello_world::main_app;

#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions {
        android_app: Some(app),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_ui_native("Egui Android", options, |u, f| main_app(u, f)).unwrap()
}
