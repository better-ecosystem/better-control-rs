use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk4 as gtk;
use gtk::glib;
use sysinfo::System;
use crate::pages::page_trait::Page;

pub struct SystemInfoPage {
    pub container: gtk::Box,
}

impl SystemInfoPage {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 12);
        let mut sys = System::new_all();
        sys.refresh_all();

        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let cpu_count = sys.cpus().len();

        // Remove the debug label, only show memory in GiB
        let mem_label = gtk::Label::new(Some(&format!(
            "Memory: {:.2} GiB / {:.2} GiB",
            sys.used_memory() as f64 / 1_073_741_824.0,
            sys.total_memory() as f64 / 1_073_741_824.0
        )));
        let hostname_label = gtk::Label::new(Some(&format!("Hostname: {}", hostname)));

        // CPU info: Get CPU model from /proc/cpuinfo (like fastfetch)
        let cpu_info = std::fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|content| {
                content.lines()
                    .find(|line| line.starts_with("model name"))
                    .map(|line| {
                        line.split(':')
                            .nth(1)
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
            })
            .unwrap_or_else(|| "Unknown".to_string());
        let cpu_label = gtk::Label::new(Some(&format!("CPU: {} ({} cores)", cpu_info, cpu_count)));

        // Fetch distro name from /etc/os-release
        let distro = std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|content| {
                content.lines()
                    .find(|line| line.starts_with("PRETTY_NAME="))
                    .map(|line| line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
            })
            .unwrap_or_else(|| "Unknown".to_string());
        let distro_label = gtk::Label::new(Some(&format!("Distro: {}", distro)));

        // Remove sys.storage() code, only use statvfs for disk info
        // Fallback: get disk usage for root ("/") using std::fs and nix crate
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
        let disk_label = gtk::Label::new(Some(&disk_info));

        // GPU info: Try to get from lspci (Linux only, like fastfetch)
        let gpu_info = std::process::Command::new("sh")
            .arg("-c")
            .arg("lspci | grep -i 'vga\\|3d\\|2d' || echo GPU: Unknown")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|out| out.trim().to_string())
            .unwrap_or_else(|| "GPU: Unknown".to_string());
        let gpu_label = gtk::Label::new(Some(&format!("GPU: {}", gpu_info)));

        container.append(&hostname_label);
        container.append(&cpu_label);
        container.append(&mem_label);
        container.append(&distro_label);
        container.append(&disk_label);
        container.append(&gpu_label);

        Self { container }
    }
}

impl Page for SystemInfoPage {
    fn get_widget(&self) -> &gtk::Widget {
        self.container.upcast_ref::<gtk::Widget>()
    }
    fn refresh(&mut self) {}
}
