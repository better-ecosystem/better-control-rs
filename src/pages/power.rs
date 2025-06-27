use adw::prelude::*;
use adw::{ActionRow, ExpanderRow, PreferencesGroup};
use gtk4::{Box, Image, Label, Orientation, ToggleButton};
use std::process::Command;

pub fn create_power_page() -> Box {
    let container = Box::new(Orientation::Vertical, 12);
    container.set_margin_top(20);
    container.set_margin_bottom(20);
    container.set_margin_start(20);
    container.set_margin_end(20);

    // Power Profile Switcher
    let group = PreferencesGroup::new();
    group.set_title("Power Profile");
    container.append(&group);

    let power_profile_box = Box::new(Orientation::Horizontal, 0);
    power_profile_box.set_hexpand(true);
    power_profile_box.set_homogeneous(true);
    power_profile_box.add_css_class("linked");
    group.add(&power_profile_box);

    let current_profile = get_current_power_profile();

    let profiles = [
        (
            "Power Saver",
            "power-profile-power-saver-symbolic",
            "power-saver",
        ),
        ("Balanced", "power-profile-balanced-symbolic", "balanced"),
        (
            "Performance",
            "power-profile-performance-symbolic",
            "performance",
        ),
    ];

    let mut first_button: Option<ToggleButton> = None;

    for (label_text, icon_name, profile_name) in &profiles {
        let button = ToggleButton::new();
        button.set_hexpand(true);
        if let Some(first) = &first_button {
            button.set_group(Some(first));
        } else {
            first_button = Some(button.clone());
        }

        let button_box = Box::new(Orientation::Horizontal, 6);
        let icon = Image::from_icon_name(icon_name);
        let label = Label::new(Some(label_text));
        button_box.append(&icon);
        button_box.append(&label);
        button.set_child(Some(&button_box));

        if *profile_name == current_profile {
            button.set_active(true);
        }

        let profile_clone = profile_name.to_string();
        button.connect_toggled(move |btn| {
            if btn.is_active() {
                let profile_clone = profile_clone.clone();
                tokio::task::spawn_blocking(move || {
                    let _ = Command::new("sudo")
                        .arg("powerprofilesctl")
                        .arg("set")
                        .arg(&profile_clone)
                        .status();
                });
            }
        });
        power_profile_box.append(&button);
    }

    // Battery Information
    let battery_expander = ExpanderRow::builder().title("Battery Information").build();
    let battery_list = Box::new(Orientation::Vertical, 0);
    battery_expander.add_row(&battery_list);

    if let Ok(manager) = battery::Manager::new() {
        if let Ok(batteries) = manager.batteries() {
            for (idx, maybe_battery) in batteries.enumerate() {
                if let Ok(battery) = maybe_battery {
                    let state = format!("State: {:?}", battery.state());
                    let percentage = format!(
                        "Percentage: {:.2}%",
                        battery.state_of_charge().value * 100.0
                    );
                    let time_to_full = if let Some(time) = battery.time_to_full() {
                        format!("Time to full: {} seconds", time.value)
                    } else {
                        "Time to full: N/A".to_string()
                    };
                    let time_to_empty = if let Some(time) = battery.time_to_empty() {
                        format!("Time to empty: {} seconds", time.value)
                    } else {
                        "Time to empty: N/A".to_string()
                    };

                    let battery_group = PreferencesGroup::new();
                    battery_group.set_title(&format!("Battery #{}", idx + 1));
                    battery_list.append(&battery_group);

                    // State
                    let state_row = ActionRow::builder().title("State").subtitle(&state).build();
                    state_row.add_prefix(&Image::from_icon_name("battery-symbolic"));
                    battery_group.add(&state_row);

                    // Percentage
                    let percentage_row = ActionRow::builder()
                        .title("Percentage")
                        .subtitle(&percentage)
                        .build();
                    percentage_row.add_prefix(&Image::from_icon_name("battery-full-symbolic"));
                    battery_group.add(&percentage_row);

                    // Time to full
                    let time_to_full_row = ActionRow::builder()
                        .title("Time to Full")
                        .subtitle(&time_to_full)
                        .build();
                    time_to_full_row.add_prefix(&Image::from_icon_name("chronometer-symbolic"));
                    battery_group.add(&time_to_full_row);

                    // Time to empty
                    let time_to_empty_row = ActionRow::builder()
                        .title("Time to Empty")
                        .subtitle(&time_to_empty)
                        .build();
                    time_to_empty_row.add_prefix(&Image::from_icon_name("chronometer-symbolic"));
                    battery_group.add(&time_to_empty_row);
                }
            }
        }
    }

    container.append(&battery_expander);

    container
}

fn get_current_power_profile() -> String {
    if let Ok(output) = Command::new("powerprofilesctl").arg("get").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "balanced".to_string() // Default fallback
    }
}
