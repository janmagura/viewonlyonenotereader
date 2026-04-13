
use crate::parser::{parse_notebook, NotebookData};
use crate::ui::{create_main_window, AppUI};
use gtk4::prelude::*;
use libadwaita::{Application, ApplicationWindow};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppWrapper {
    app: Application,
    window: Option<ApplicationWindow>,
    notebook_data: Arc<Mutex<Option<NotebookData>>>,
}

impl AppWrapper {
    pub fn new(app: Application) -> Self {
        Self {
            app,
            window: None,
            notebook_data: Arc::new(Mutex::new(None)),
        }
    }

    pub fn show(&self, file_path: Option<String>) {
        let window = create_main_window(&self.app);
        let ui = AppUI::new(window.clone(), self.notebook_data.clone());

        // If file path provided, load it
        if let Some(path) = file_path {
            let path_buf = PathBuf::from(path);
            let ui_clone = ui.clone();
            
            glib::MainContext::default().spawn_local(async move {
                match parse_notebook(&path_buf).await {
                    Ok(data) => {
                        let mut notebook = ui_clone.notebook_data.lock().await;
                        *notebook = Some(data.clone());
                        drop(notebook);
                        ui_clone.update_sidebar(&data);
                        ui_clone.clear_content();
                    }
                    Err(e) => {
                        ui_clone.show_error(&format!("Failed to parse notebook: {}", e));
                    }
                }
            });
        } else {
            // Show file chooser dialog
            let ui_clone = ui.clone();
            glib::MainContext::default().spawn_local(async move {
                ui_clone.show_file_chooser().await;
            });
        }

        window.present();
    }
}
