
mod sidebar;
mod content_view;

pub use sidebar::Sidebar;
pub use content_view::ContentView;

use crate::parser::{NotebookData, ContentObject};
use gtk4::prelude::*;
use libadwaita::{Application, ApplicationWindow, NavigationSplitView};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_main_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OneNote Viewer")
        .default_width(1200)
        .default_height(800)
        .build();

    // Create navigation split view
    let nav_split = NavigationSplitView::builder()
        .show_content(true)
        .build();

    window.set_child(Some(&nav_split));

    window
}

#[derive(Clone)]
pub struct AppUI {
    window: ApplicationWindow,
    notebook_data: Arc<Mutex<Option<NotebookData>>>,
}

impl AppUI {
    pub fn new(window: ApplicationWindow, notebook_data: Arc<Mutex<Option<NotebookData>>>) -> Self {
        Self {
            window,
            notebook_data,
        }
    }

    pub fn update_sidebar(&self, data: &NotebookData) {
        if let Some(child) = self.window.child() {
            if let Some(nav_split) = child.downcast_ref::<NavigationSplitView>() {
                let sidebar = Sidebar::new(data);
                nav_split.set_sidebar(Some(&sidebar.container()));
                
                // Connect sidebar selection
                let content_view = ContentView::new();
                nav_split.set_content(Some(&content_view.container()));
                
                sidebar.connect_page_selected(move |page| {
                    content_view.display_page(&page);
                });
            }
        }
    }

    pub fn clear_content(&self) {
        // Clear any existing content
    }

    pub fn show_error(&self, message: &str) {
        use libadwaita::Toast;
        
        if let Some(child) = self.window.child() {
            if let Some(nav_split) = child.downcast_ref::<NavigationSplitView>() {
                if let Some(content) = nav_split.content() {
                    if let Some(scrolled) = content.first_child() {
                        if let Some(box_container) = scrolled.first_child() {
                            // Show toast notification
                            let toast = Toast::builder()
                                .title(message)
                                .timeout(5)
                                .build();
                            
                            // Try to find and use AdwToastOverlay if available
                            // For now, just log the error
                            log::error!("Error: {}", message);
                        }
                    }
                }
            }
        }
        
        // Also show a dialog for critical errors
        let dialog = gtk4::AlertDialog::builder()
            .message("Error Loading Notebook")
            .detail(message)
            .build();
        
        dialog.show(Some(&self.window));
    }

    pub async fn show_file_chooser(&self) {
        let dialog = gtk4::FileDialog::builder()
            .title("Open OneNote Notebook")
            .modal(true)
            .build();

        // Add filters for .one and .onetoc2 files
        let filter_one = gtk4::FileFilter::new();
        filter_one.set_name(Some("OneNote Files (.one)"));
        filter_one.add_pattern("*.one");

        let filter_onetoc2 = gtk4::FileFilter::new();
        filter_onetoc2.set_name(Some("OneNote Table of Contents (.onetoc2)"));
        filter_onetoc2.add_pattern("*.onetoc2");

        let filters = gio::ListStore::new::<gtk4::FileFilter>();
        filters.append(&filter_one);
        filters.append(&filter_onetoc2);
        dialog.set_filters(Some(&filters));

        let window = self.window.clone();
        let notebook_data = self.notebook_data.clone();

        match dialog.open_future(Some(&window)).await {
            Ok(file) => {
                if let Some(path) = file.path() {
                    match crate::parser::parse_notebook(&path).await {
                        Ok(data) => {
                            let mut notebook = notebook_data.lock().await;
                            *notebook = Some(data.clone());
                            drop(notebook);
                            self.update_sidebar(&data);
                            self.clear_content();
                        }
                        Err(e) => {
                            self.show_error(&format!("Failed to parse notebook: {}", e));
                        }
                    }
                }
            }
            Err(_) => {
                // User cancelled the dialog
                log::info!("File chooser cancelled");
            }
        }
    }
}
