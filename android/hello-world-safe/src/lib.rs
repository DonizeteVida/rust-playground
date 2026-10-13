#[unsafe(no_mangle)]
fn android_main(android_app: android_activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    log::info!("Hello World!!!");

    loop {
        android_app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            log::info!("Event: {event:?}");
        });

        if let Ok(mut events) = android_app.input_events_iter() {
            loop {
                let read_input = events.next(|event| {
                    log::info!("Input event: {event:?}");

                    android_activity::InputStatus::Handled
                });

                if !read_input {
                    break;
                }
            }
        }
    }
}
