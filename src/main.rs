mod app;
mod window;
mod pages;

use adw::prelude::*;
use app::BetterControlApp;
use window::build_main_window;

#[tokio::main]
async fn main() {
    // let _ = gio::resources_register_include!("resources/ui/window.ui");
    let app = BetterControlApp::new();
    app.connect_activate(|app| {
        let window = build_main_window(app);
        window.present();
    });
    app.run();
}
