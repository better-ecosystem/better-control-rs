use crate::pages::async_page::AsyncPage;
use crate::pages::page_trait::Page;
use adw::prelude::*;
use gtk4 as gtk;
use std::sync::mpsc;
use sysinfo::System;

pub struct SystemInfoPage {
    pub container: gtk::Box,
}

impl SystemInfoPage {
    pub fn new() -> Self {
        let async_page = AsyncPage::new("System Info", |container| async move {
            let (sender, receiver) = mpsc::channel::<Vec<String>>();

            std::thread::spawn(move || {
                let mut sys = System::new_all();
                sys.refresh_all();

                let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
                let cpu_count = sys.cpus().len();

                let mem_label = format!(
                    "Memory: {:.2} GiB / {:.2} GiB",
                    sys.used_memory() as f64 / 1_073_741_824.0,
                    sys.total_memory() as f64 / 1_073_741_824.0
                );
                let hostname_label = format!("Hostname: {}", hostname);

                let cpu_info = std::fs::read_to_string("/proc/cpuinfo")
                    .ok()
                    .and_then(|content| {
                        content
                            .lines()
                            .find(|line| line.starts_with("model name"))
                            .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
                    })
                    .unwrap_or_else(|| "Unknown".to_string());
                let cpu_label = format!("CPU: {} ({} cores)", cpu_info, cpu_count);

                let distro = std::fs::read_to_string("/etc/os-release")
                    .ok()
                    .and_then(|content| {
                        content
                            .lines()
                            .find(|line| line.starts_with("PRETTY_NAME="))
                            .map(|line| {
                                line.trim_start_matches("PRETTY_NAME=")
                                    .trim_matches('"')
                                    .to_string()
                            })
                    })
                    .unwrap_or_else(|| "Unknown".to_string());
                let distro_label = format!("Distro: {}", distro);

                let disk_info = nix::sys::statvfs::statvfs("/")
                    .map(|stat| {
                        let total = stat.blocks() * stat.fragment_size() as u64;
                        let free = stat.blocks_available() * stat.fragment_size() as u64;
                        let used = total - free;
                        format!(
                            "Disk: {:.2} GiB / {:.2} GiB",
                            used as f64 / 1_073_741_824.0,
                            total as f64 / 1_073_741_824.0
                        )
                    })
                    .unwrap_or_else(|_| "Disk: Unknown".to_string());

                let gpu_info = std::process::Command::new("sh")
                    .arg("-c")
                    .arg("lspci | grep -i 'vga\\|3d\\|2d' | head -n1 || echo GPU: Unknown")
                    .output()
                    .ok()
                    .and_then(|output| String::from_utf8(output.stdout).ok())
                    .map(|out| {
                        let line = out.trim();
                        if let Some(idx) = line.find(':') {
                            let after_colon = &line[idx + 1..];
                            if let Some(name_start) = after_colon.find("controller:") {
                                after_colon[name_start + "controller:".len()..]
                                    .trim()
                                    .to_string()
                            } else if let Some(name_start) = after_colon.find("controller") {
                                after_colon[name_start + "controller".len()..]
                                    .trim()
                                    .to_string()
                            } else {
                                after_colon.trim().to_string()
                            }
                        } else {
                            line.to_string()
                        }
                    })
                    .unwrap_or_else(|| "Unknown".to_string());
                let gpu_label = format!("GPU: {}", gpu_info);

                let results = vec![
                    hostname_label,
                    cpu_label,
                    mem_label,
                    distro_label,
                    disk_info,
                    gpu_label,
                ];
                sender.send(results).expect("Failed to send system info");
            });

            glib::MainContext::default().spawn_local({
                let container = container.clone();
                async move {
                    if let Ok(results) = receiver.recv() {
                        while let Some(child) = container.first_child() {
                            container.remove(&child);
                        }

                        let group = adw::PreferencesGroup::new();
                        group.set_margin_top(32);
                        group.set_margin_bottom(32);
                        group.set_margin_start(32);
                        group.set_margin_end(32);

                        group.set_halign(gtk::Align::Center);
                        group.set_valign(gtk::Align::Center);

                        let icon_names = [
                            "computer-symbolic",                 // hostname
                            "cpu-symbolic",                      // cpu
                            "memory-symbolic",                   // memory
                            "applications-engineering-symbolic", // distro
                            "drive-harddisk-symbolic",           // disk
                            "video-display-symbolic",            // gpu
                        ];

                        for (i, result_str) in results.iter().enumerate() {
                            let parts: Vec<&str> = result_str.splitn(2, ':').collect();
                            let title = parts[0].trim();
                            let subtitle = if parts.len() > 1 {
                                parts[1].trim()
                            } else {
                                ""
                            };

                            let row = adw::ActionRow::builder()
                                .title(title)
                                .subtitle(subtitle)
                                .build();

                            let icon = gtk::Image::from_icon_name(
                                icon_names
                                    .get(i)
                                    .copied()
                                    .unwrap_or("dialog-question-symbolic"),
                            );
                            row.add_prefix(&icon);
                            group.add(&row);
                        }
                        container.append(&group);
                    }
                }
            });
        });

        Self {
            container: async_page.container,
        }
    }
}

impl Page for SystemInfoPage {
    fn get_widget(&self) -> &gtk::Widget {
        self.container.upcast_ref::<gtk::Widget>()
    }
    fn refresh(&mut self) {}
}
