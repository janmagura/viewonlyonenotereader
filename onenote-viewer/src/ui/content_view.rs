
use crate::parser::{Page, ContentObject, TextRun, ImageData, AttachmentData};
use gtk4::prelude::*;
use gtk4::{Box, Label, ScrolledWindow, Image, Button, Orientation, Separator};
use std::path::PathBuf;

pub struct ContentView {
    container: ScrolledWindow,
    content_box: Box,
}

impl ContentView {
    pub fn new() -> Self {
        let content_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();

        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&content_box)
            .build();

        Self {
            container: scrolled,
            content_box,
        }
    }

    pub fn container(&self) -> &ScrolledWindow {
        &self.container
    }

    pub fn display_page(&self, page: &Page) {
        // Clear existing content
        while let Some(child) = self.content_box.first_child() {
            self.content_box.remove(&child);
        }

        // Add page title
        let title_label = Label::builder()
            .label(&page.title)
            .use_markup(true)
            .wrap(true)
            .wrap_mode(gtk4::pango::WrapMode::WordChar)
            .xalign(0.0)
            .build();
        
        title_label.add_css_class("title");
        title_label.add_css_class("heading");
        self.content_box.append(&title_label);

        let separator = Separator::builder()
            .orientation(Orientation::Horizontal)
            .build();
        self.content_box.append(&separator);

        // Render content objects
        for content in &page.content {
            match content {
                ContentObject::Text(text_run) => {
                    self.render_text(text_run);
                }
                ContentObject::Image(image_data) => {
                    self.render_image(image_data);
                }
                ContentObject::Attachment(attachment) => {
                    self.render_attachment(attachment);
                }
                ContentObject::Unsupported(reason) => {
                    self.render_unsupported(reason);
                }
            }
        }

        // If no content, show placeholder
        if page.content.is_empty() {
            let placeholder = Label::builder()
                .label("No content available for this page")
                .wrap(true)
                .xalign(0.5)
                .build();
            placeholder.add_css_class("dim-label");
            self.content_box.append(&placeholder);
        }
    }

    fn render_text(&self, text_run: &TextRun) {
        let mut markup = String::new();

        // Apply formatting
        if text_run.is_header {
            markup.push_str("<span size='large' weight='bold'>");
        } else if text_run.is_bold {
            markup.push_str("<b>");
        }
        if text_run.is_italic {
            markup.push_str("<i>");
        }

        // Add indentation for lists
        if text_run.list_level > 0 {
            for _ in 0..text_run.list_level {
                markup.push_str("    ");
            }
            markup.push_str("• ");
        }

        // Escape HTML entities in text
        let escaped_text = text_run.text
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        markup.push_str(&escaped_text);

        // Close tags
        if text_run.is_italic {
            markup.push_str("</i>");
        }
        if text_run.is_header {
            markup.push_str("</span>");
        } else if text_run.is_bold {
            markup.push_str("</b>");
        }

        let label = Label::builder()
            .use_markup(true)
            .label(&markup)
            .wrap(true)
            .wrap_mode(gtk4::pango::WrapMode::WordChar)
            .xalign(0.0)
            .selectable(true)
            .build();

        self.content_box.append(&label);
    }

    fn render_image(&self, image_data: &ImageData) {
        // Try to create a GdkPixbuf from the image data
        match gtk4::gdk_pixbuf::Pixbuf::from_buffer(&image_data.data) {
            Ok(pixbuf) => {
                // Scale if dimensions are provided
                let scaled_pixbuf = if let (Some(width), Some(height)) = (image_data.width, image_data.height) {
                    pixbuf.scale_simple(
                        width as i32.min(800),
                        height as i32.min(600),
                        gtk4::gdk_pixbuf::InterpType::Bilinear,
                    )
                } else {
                    Some(pixbuf)
                };

                if let Some(final_pixbuf) = scaled_pixbuf {
                    let image = Image::builder()
                        .pixbuf(&final_pixbuf)
                        .build();
                    
                    // Make it clickable to view full size (future enhancement)
                    let event_box = gtk4::EventControllerMotion::new();
                    image.add_controller(event_box);
                    
                    self.content_box.append(&image);
                }
            }
            Err(e) => {
                log::warn!("Failed to load image: {}", e);
                let label = Label::builder()
                    .label("Image could not be displayed")
                    .wrap(true)
                    .build();
                label.add_css_class("dim-label");
                self.content_box.append(&label);
            }
        }
    }

    fn render_attachment(&self, attachment: &AttachmentData) {
        let box_container = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .build();

        // Create icon based on file type
        let icon_name = self.get_icon_for_file(&attachment.filename);
        let icon = Image::builder()
            .icon_name(icon_name)
            .pixel_size(32)
            .build();

        // Create label with filename and size
        let size_str = format_size(attachment.size);
        let label = Label::builder()
            .label(&format!("{}\n{}", attachment.filename, size_str))
            .xalign(0.0)
            .wrap(true)
            .build();

        // Create button to open attachment
        let button = Button::builder()
            .label("Open")
            .halign(gtk4::Align::End)
            .valign(gtk4::Align::Center)
            .build();

        let filename = attachment.filename.clone();
        let data = attachment.data.clone();
        button.connect_clicked(move |_| {
            // Save to temp file and open
            if let Ok(temp_dir) = std::env::var("TMPDIR") {
                let temp_path = PathBuf::from(temp_dir).join(&filename);
                if let Ok(_) = std::fs::write(&temp_path, &data) {
                    // Open with default application
                    let uri = format!("file://{}", temp_path.display());
                    if let Ok(_) = gtk4::gio::AppInfo::launch_default_for_uri(&uri, None::<&gtk4::gio::AppLaunchContext>) {
                        log::info!("Opened attachment: {}", filename);
                    }
                }
            }
        });

        box_container.append(&icon);
        box_container.append(&label);
        box_container.append(&button);

        self.content_box.append(&box_container);
    }

    fn render_unsupported(&self, reason: &str) {
        let label = Label::builder()
            .label(reason)
            .wrap(true)
            .xalign(0.0)
            .build();
        label.add_css_class("warning");
        label.add_css_class("dim-label");
        self.content_box.append(&label);
    }

    fn get_icon_for_file(&self, filename: &str) -> &str {
        let extension = filename.split('.').last().unwrap_or("").to_lowercase();
        match extension.as_str() {
            "pdf" => "application-pdf",
            "doc" | "docx" => "application-msword",
            "xls" | "xlsx" => "application-vnd.ms-excel",
            "ppt" | "pptx" => "application-vnd.ms-powerpoint",
            "txt" => "text-plain",
            "jpg" | "jpeg" | "png" | "gif" => "image-x-generic",
            "zip" | "tar" | "gz" | "rar" => "package-x-generic",
            _ => "text-x-generic",
        }
    }
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}
