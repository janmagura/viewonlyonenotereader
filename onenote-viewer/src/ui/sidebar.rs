
use crate::parser::{NotebookData, Section, Page};
use gtk4::prelude::*;
use gtk4::{Box, Label, ScrolledWindow, TreeView, TreeViewColumn, CellRendererText};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sidebar {
    container: ScrolledWindow,
    tree_view: TreeView,
    page_selected_callback: Rc<RefCell<Option<Box<dyn Fn(Page)>>>>,
}

impl Sidebar {
    pub fn new(data: &NotebookData) -> Self {
        let tree_view = TreeView::new();
        tree_view.set_headers_visible(false);

        // Create columns
        let column = TreeViewColumn::new();
        let renderer = CellRendererText::new();
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", 0);
        tree_view.append_column(&column);

        // Create tree store: [name (String), type (i32), page_data (Page)]
        // Type: 0 = Notebook, 1 = Section, 2 = Page
        let store = gtk4::TreeStore::new(&[
            String::static_type(),
            i32::static_type(),
            gtk4::glib::Type::BOXED,
        ]);

        // Add notebook root
        let notebook_iter = store.insert_with_values(None, None, &[
            (0, &data.name),
            (1, &0i32),
            (2, &None::<Page>),
        ]);

        // Add sections
        for section in &data.sections {
            let section_iter = store.insert_with_values(Some(&notebook_iter), None, &[
                (0, &section.name),
                (1, &1i32),
                (2, &None::<Page>),
            ]);

            // Add pages
            for page in &section.pages {
                store.insert_with_values(Some(&section_iter), None, &[
                    (0, &page.title),
                    (1, &2i32),
                    (2, &Some(page.clone())),
                ]);
            }
        }

        tree_view.set_model(Some(&store));
        tree_view.expand_all();

        // Wrap in scrolled window
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&tree_view)
            .build();

        Self {
            container: scrolled,
            tree_view,
            page_selected_callback: Rc::new(RefCell::new(None)),
        }
    }

    pub fn container(&self) -> &ScrolledWindow {
        &self.container
    }

    pub fn connect_page_selected<F>(&self, callback: F)
    where
        F: Fn(Page) + 'static,
    {
        *self.page_selected_callback.borrow_mut() = Some(Box::new(callback));

        let callback_clone = self.page_selected_callback.clone();
        
        self.tree_view.connect_row_activated(move |tree_view, path, _column| {
            if let Some(model) = tree_view.model() {
                if let Some(iter) = model.iter(&path) {
                    let value = model.value(&iter, 2);
                    if let Some(page_boxed) = value.get::<Option<Page>>() {
                        if let Some(page) = page_boxed {
                            if let Some(ref cb) = *callback_clone.borrow() {
                                cb(page.clone());
                            }
                        }
                    }
                }
            }
        });
    }
}
