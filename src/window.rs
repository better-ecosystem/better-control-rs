use adw::prelude::*;
use adw::{ApplicationWindow, ViewStack, ViewSwitcher};
use gtk4::glib;
use gtk4::Box as GtkBox;

pub fn build_main_window(app: &adw::Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Better Control RS")
        .default_width(900)
        .default_height(600)
        .build();

    // Use a vertical GtkBox to hold the view switcher and content
    let vbox = GtkBox::new(gtk4::Orientation::Vertical, 0);

    let view_stack = ViewStack::builder().build();
    let view_switcher = ViewSwitcher::builder().stack(&view_stack).build();

    // Add all pages in alphabetical order, no header bar
    let audio = gtk4::Label::new(Some("Audio Management Page (empty)"));
    view_stack.add_titled(&audio, Some("audio"), "Audio");

    let bluetooth = gtk4::Label::new(Some("Bluetooth Management Page (empty)"));
    view_stack.add_titled(&bluetooth, Some("bluetooth"), "Bluetooth");

    let displays = gtk4::Label::new(Some("Display Management Page (empty)"));
    view_stack.add_titled(&displays, Some("displays"), "Displays");

    let power = gtk4::Label::new(Some("Power Management Page (empty)"));
    view_stack.add_titled(&power, Some("power"), "Power");

    let system_info = gtk4::Label::new(Some("System Info Page (empty)"));
    view_stack.add_titled(&system_info, Some("system_info"), "System Info");

    let wifi = gtk4::Label::new(Some("WiFi Management Page (empty)"));
    view_stack.add_titled(&wifi, Some("wifi"), "WiFi");

    vbox.append(&view_switcher);
    vbox.append(&view_stack);
    window.set_content(Some(&vbox));
    window
}
