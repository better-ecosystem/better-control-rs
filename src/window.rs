use crate::pages::power;
use crate::pages::system_info::SystemInfoPage;
use adw::prelude::*;
use adw::{ApplicationWindow, ViewStack, ViewSwitcher};
use gtk4::Box as GtkBox;
use gtk4::EventControllerKey;
use gtk4::gdk;
use gtk4::glib;

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
    // Add margin to the view_switcher so it doesn't touch the window edges
    view_switcher.set_margin_top(12);
    view_switcher.set_margin_bottom(12);
    view_switcher.set_margin_start(12);
    view_switcher.set_margin_end(12);

    // Add all pages in alphabetical order, no header bar
    let audio = gtk4::Label::new(Some("Audio Management Page (empty)"));
    view_stack.add_titled(&audio, Some("audio"), "Audio");
    let page = view_stack.page(&audio);
    page.set_icon_name(Some("audio-volume-high-symbolic"));

    let bluetooth = gtk4::Label::new(Some("Bluetooth Management Page (empty)"));
    view_stack.add_titled(&bluetooth, Some("bluetooth"), "Bluetooth");
    let page = view_stack.page(&bluetooth);
    page.set_icon_name(Some("bluetooth-symbolic"));

    let displays = gtk4::Label::new(Some("Display Management Page (empty)"));
    view_stack.add_titled(&displays, Some("displays"), "Displays");
    let page = view_stack.page(&displays);
    page.set_icon_name(Some("display-symbolic"));

    let power = power::create_power_page();
    view_stack.add_titled(&power, Some("power"), "Power");
    let page = view_stack.page(&power);
    page.set_icon_name(Some("battery-ac-adapter-symbolic"));

    // Use async System Info page
    let system_info_page = SystemInfoPage::new();
    view_stack.add_titled(
        &system_info_page.container,
        Some("system_info"),
        "System Info",
    );
    let page = view_stack.page(&system_info_page.container);
    page.set_icon_name(Some("dialog-information-symbolic"));

    let network = gtk4::Label::new(Some("Network Management Page (empty)"));
    view_stack.add_titled(&network, Some("network"), "Network");
    let page = view_stack.page(&network);
    page.set_icon_name(Some("preferences-system-network-symbolic"));

    vbox.append(&view_switcher);
    vbox.append(&view_stack);
    window.set_content(Some(&vbox));

    // Add keyboard shortcuts for switching pages
    let key_controller = EventControllerKey::new();
    let view_stack_clone = view_stack.clone();
    key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        if state.contains(gdk::ModifierType::CONTROL_MASK) {
            match keyval.to_unicode() {
                Some('1') => view_stack_clone.set_visible_child_name("audio"),
                Some('2') => view_stack_clone.set_visible_child_name("bluetooth"),
                Some('3') => view_stack_clone.set_visible_child_name("displays"),
                Some('4') => view_stack_clone.set_visible_child_name("power"),
                Some('5') => view_stack_clone.set_visible_child_name("system_info"),
                Some('6') => view_stack_clone.set_visible_child_name("network"),
                _ => return glib::Propagation::Proceed,
            }
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });
    window.add_controller(key_controller);

    window
}
