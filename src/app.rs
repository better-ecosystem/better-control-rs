use adw::prelude::*;
use adw::Application;
use gtk4::gio;

pub struct BetterControlApp;

impl BetterControlApp {
    pub fn new() -> Application {
        Application::builder()
            .application_id("rs.better.control")
            .build()
    }
}
