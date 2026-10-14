use hello_world::main_app;

fn main() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_ui_native("Egui Desktop", options, |u, f| main_app(u, f)).unwrap()
}
