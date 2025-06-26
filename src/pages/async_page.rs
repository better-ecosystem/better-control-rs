use glib::MainContext;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation};
use std::rc::Rc;

/// A generic async-loading page container for GTK4/Adwaita apps.
pub struct AsyncPage {
    pub container: GtkBox,
}

impl AsyncPage {
    /// Create a new async page.
    ///
    /// `title` is shown while loading.
    /// `loader` is an async closure that receives the container to populate.
    pub fn new<F, Fut>(title: &str, loader: F) -> Self
    where
        F: FnOnce(GtkBox) -> Fut + 'static,
        Fut: std::future::Future<Output = ()> + 'static,
    {
        let container = GtkBox::new(Orientation::Vertical, 12);
        let label = Label::new(Some(&format!("Loading {}...", title)));
        container.append(&label);

        let loaded = Rc::new(std::cell::Cell::new(false));
        let loaded_clone = loaded.clone();
        let container_clone = container.clone();

        MainContext::default().spawn_local(async move {
            loader(container_clone).await;
            loaded_clone.set(true);
        });

        Self { container }
    }
}
