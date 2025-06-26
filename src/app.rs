use adw::Application;

pub struct BetterControlApp;

impl BetterControlApp {
    pub fn new() -> Application {
        Application::builder()
            .application_id("rs.better.control")
            .build()
    }
}
