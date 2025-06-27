mod app;
mod pages;
mod window;

use adw::prelude::*;
use app::BetterControlApp;
use gtk4::glib;
use window::{build_main_window, setup_pages};

#[tokio::main]
async fn main() {
    let app = BetterControlApp::new();

    app.connect_activate(|app| {
        let (window, view_stack) = build_main_window(app);
        window.present();

        // Spawn a new task to load the pages asynchronously
        glib::MainContext::default().spawn_local(async move {
            setup_pages(&view_stack, &window);
        });
    });

    app.run();
}
