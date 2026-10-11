#[cfg(not(target_os = "android"))]
compile_error!("This code is meant to be compiled only for Android");

unsafe extern "C" fn my_on_resume_custom_callback(activity: *mut ndk_sys::ANativeActivity) {
    log::info!("ON RESUME IS CALLED");

    unsafe {
        ndk_sys::ANativeActivity_finish(activity);
    }
}

unsafe extern "C" fn my_on_destroy_custom_callback(activity: *mut ndk_sys::ANativeActivity) {
    log::info!("ON DESTROY IS CALLED");
}

#[unsafe(no_mangle)]
fn ANativeActivity_onCreate(
    activity: *mut ndk_sys::ANativeActivity,
    savedState: *mut ::std::os::raw::c_void,
    savedStateSize: usize,
) {
    static APP_ONCE: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    APP_ONCE.get_or_init(|| {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Info),
        );
    });

    let sdk_version = unsafe { (*activity).sdkVersion };

    log::info!("Hello World: {}", sdk_version);

    unsafe {
        let callbacks = (*activity).callbacks;
        (*callbacks).onResume = Some(my_on_resume_custom_callback);
        (*callbacks).onDestroy = Some(my_on_destroy_custom_callback);
    }
}
