fn main() -> eframe::Result {
    eframe::run_ui_native("My app", Default::default(), |ui, frame| {
        ui.label("Hello World");
    })
}
