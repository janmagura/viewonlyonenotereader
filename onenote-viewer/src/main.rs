
mod app;
mod parser;
mod ui;
mod utils;

use gtk4::prelude::*;
use libadwaita::Application;
use std::env;

fn main() -> glib::ExitCode {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    // Create the application
    let app = Application::builder()
        .application_id("com.example.onenote-viewer")
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // Get CLI argument if provided
    let file_path = env::args().nth(1);

    app.connect_startup(|_| {
        libadwaita::init();
    });

    app.connect_activate(move |app| {
        let app_wrapper = app::AppWrapper::new(app.clone());
        app_wrapper.show(file_path.clone());
    });

    app.connect_open(move |app, files, _hint| {
        if let Some(file) = files.first() {
            let app_wrapper = app::AppWrapper::new(app.clone());
            if let Some(path) = file.path() {
                app_wrapper.show(Some(path.to_string_lossy().to_string()));
            }
        }
    });

    app.run_with_args(&std::env::args().collect::<Vec<_>>())
}
