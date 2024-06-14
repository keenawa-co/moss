fn main() {
    #[cfg(target_os = "macos")]
    macos::build();
}

mod macos {
    use std::path::PathBuf;

    pub(super) fn build() {
        gen_dispatch_bindgen()
    }

    fn gen_dispatch_bindgen() {
        println!("cargo:rustc-link-lib=framework=System");
        println!("cargo:rerun-if-changed=src/platform/mac/dispatch.h");

        let bindings = bindgen::Builder::default()
            .header("src/platform/mac/dispatch.h")
            .allowlist_var("_dispatch_main_q")
            .allowlist_var("_dispatch_source_type_data_add")
            .allowlist_var("DISPATCH_QUEUE_PRIORITY_HIGH")
            .allowlist_var("DISPATCH_TIME_NOW")
            .allowlist_function("dispatch_get_global_queue")
            .allowlist_function("dispatch_async_f")
            .allowlist_function("dispatch_after_f")
            .allowlist_function("dispatch_time")
            .allowlist_function("dispatch_source_merge_data")
            .allowlist_function("dispatch_source_create")
            .allowlist_function("dispatch_source_set_event_handler_f")
            .allowlist_function("dispatch_resume")
            .allowlist_function("dispatch_suspend")
            .allowlist_function("dispatch_source_cancel")
            .allowlist_function("dispatch_set_context")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .layout_tests(false)
            .generate()
            .expect("unable to generate bindings");

        let out_path = PathBuf::from("src/platform/mac");
        bindings
            .write_to_file(out_path.join("dispatch_sys.rs"))
            .expect("couldn't write dispatch bindings");
    }
}
