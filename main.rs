use std::env;
use std::process::Command;
use sysinfo::{System, Disks};
use local_ip_address::linux::list_afinet_netifas;
use clap::{Arg, Command as ClapCommand};
use whoami;

fn get_os_info() -> String {
    let os = std::env::consts::OS;
    let mut sys = System::new_all();
    sys.refresh_all();
    match os {
        "linux" => {
            if let Some(os_name) = sysinfo::System::name() {
                return os_name;
            }
            if let Ok(os_release) = std::fs::read_to_string("/etc/os-release") {
                for line in os_release.lines() {
                    if line.starts_with("PRETTY_NAME=") {
                        return line[12..].trim_matches('"').to_string();
                    }
                }
            }
            "Linux".to_string()
        }
        _ => format!("{} {}", os, sysinfo::System::os_version().unwrap_or("Unknown".to_string())),
    }
}

fn get_kernel_info() -> String {
    sysinfo::System::kernel_version().unwrap_or("Unknown".to_string())
}

fn get_battery_info() -> String {
    if let Ok(output) = Command::new("upower")
        .arg("-i")
        .arg("/org/freedesktop/UPower/devices/battery_BAT0")
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut percentage = "Unknown".to_string();
        let mut state = "Unknown";
        for line in output_str.lines() {
            if line.contains("percentage:") {
                percentage = line.split(":").nth(1).unwrap_or("Unknown").trim().to_string();
            }
            if line.contains("state:") {
                state = line.split(":").nth(1).unwrap_or("Unknown").trim();
            }
        }
        format!("{} [{}]", percentage, state)
    } else {
        "No Battery".to_string()
    }
}

fn get_disk_usage_info() -> Vec<(String, u64, u64)> {
    let mut disks = Disks::new_with_refreshed_list();
    let mut disk_usage = Vec::new();
    for disk in &disks {
        let mount_point = disk.mount_point().to_string_lossy().to_string();
        if !mount_point.starts_with("/snap") {
            let total = disk.total_space();
            let used = total - disk.available_space();
            disk_usage.push((mount_point, total, used));
        }
    }
    disk_usage
}

fn get_swap_info() -> (u64, u64) {
    let mut sys = System::new_all();
    sys.refresh_memory();
    (sys.total_swap(), sys.used_swap())
}

fn get_arch_info() -> String {
    std::env::consts::ARCH.to_string()
}

fn get_cpu_info() -> (String, usize) {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    let cpu = sys.global_cpu_info().brand().to_string();
    let cores = sys.cpus().len();
    (cpu, cores)
}

fn get_local_ip_info() -> String {
    if let Ok(network_interfaces) = list_afinet_netifas() {
        for (_name, ip) in network_interfaces {
            if let std::net::IpAddr::V4(ipv4) = ip {
                if !ipv4.is_loopback() {
                    return ipv4.to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_memory_info() -> (u64, u64) {
    let mut sys = System::new_all();
    sys.refresh_memory();
    (sys.total_memory(), sys.used_memory())
}

fn get_locale_info() -> String {
    env::var("LANG").unwrap_or("Unknown".to_string())
}

fn get_uptime_info() -> String {
    let uptime = sysinfo::System::uptime();
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;
    format!("{}h {}m {}s", hours, minutes, seconds)
}

fn get_hostname_info() -> String {
    whoami::fallible::hostname().unwrap_or("Unknown".to_string())
}

fn get_user_info() -> String {
    whoami::username()
}

fn get_gpu_info() -> String {
    if let Ok(output) = Command::new("lspci").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("VGA") || line.contains("3D controller") {
                if let Some(pos) = line.find(": ") {
                    return line[pos + 2..].trim().to_string();
                }
            }
        }
    }
    "No GPU found".to_string()
}

fn get_package_manager_info() -> Vec<(String, u64)> {
    let package_managers = [
        ("dpkg", "dpkg-query -f '.\n' -W 2>/dev/null | wc -l"),
        ("apt", "apt list --installed 2>/dev/null | wc -l"),
        ("rpm", "rpm -qa 2>/dev/null | wc -l"),
        ("pacman", "pacman -Q 2>/dev/null | wc -l"),
        ("dnf", "dnf list installed 2>/dev/null | wc -l"),
        ("snap", "snap list 2>/dev/null | wc -l"),
        ("flatpak", "flatpak list 2>/dev/null | wc -l"),
    ];
    let mut installed_packages = Vec::new();
    for (manager, cmd) in package_managers {
        if let Ok(output) = Command::new("sh").arg("-c").arg(cmd).output() {
            if let Ok(count) = String::from_utf8_lossy(&output.stdout).trim().parse::<u64>() {
                if count > 0 {
                    installed_packages.push((manager.to_string(), count));
                }
            }
        }
    }
    installed_packages
}

fn get_resolution() -> String {
    if let Ok(output) = Command::new("xrandr").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains('*') {
                if let Some(res) = line.split_whitespace().next() {
                    return res.to_string();
                }
            }
        }
    }
    if let Ok(output) = Command::new("xdpyinfo").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("dimensions:") {
                if let Some(res) = line.split_whitespace().nth(1) {
                    return res.to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_desktop_environment() -> String {
    let desktop_session = env::var("DESKTOP_SESSION").unwrap_or_default().to_lowercase();
    match desktop_session.as_str() {
        s if s.contains("zorin") => "Zorin".to_string(),
        s if s.contains("gnome") => "GNOME".to_string(),
        s if s.contains("kde") => "KDE Plasma".to_string(),
        s if s.contains("xfce") => "XFCE".to_string(),
        s if s.contains("lxqt") => "LXQt".to_string(),
        s if s.contains("lxde") => "LXDE".to_string(),
        s if s.contains("mate") => "MATE".to_string(),
        s if s.contains("cinnamon") => "Cinnamon".to_string(),
        s if s.contains("budgie") => "Budgie".to_string(),
        s if s.contains("pantheon") => "Pantheon".to_string(),
        "" => "Unknown".to_string(),
        _ => desktop_session
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_uppercase().to_string() } else { c.to_string() })
            .collect(),
    }
}

fn get_window_manager() -> String {
    if let Ok(xdg_session) = env::var("XDG_SESSION_TYPE") {
        if xdg_session.to_lowercase() == "wayland" {
            return "Wayland".to_string();
        }
        if xdg_session.to_lowercase() == "x11" {
            if let Ok(output) = Command::new("wmctrl").arg("-m").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("Name:") {
                        if let Some(wm) = line.split(":").nth(1) {
                            return wm.trim().to_string();
                        }
                    }
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_window_manager_theme() -> String {
    let desktop_session = env::var("DESKTOP_SESSION").unwrap_or_default().to_lowercase();
    if desktop_session.contains("gnome") || desktop_session.contains("zorin") {
        if let Ok(output) = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.wm.preferences", "theme"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.trim().trim_matches('\'').to_string();
        }
    } else if desktop_session.contains("kde") {
        if let Ok(output) = Command::new("kreadconfig5")
            .args(["--group", "WM", "--key", "theme"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.trim().to_string();
        }
    }
    "Unknown".to_string()
}

fn get_gtk_theme() -> String {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return output_str.trim().trim_matches('\'').to_string();
    }
    "Unknown".to_string()
}

fn get_icon_theme() -> String {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "icon-theme"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return output_str.trim().trim_matches('\'').to_string();
    }
    "Unknown".to_string()
}

fn get_terminal() -> String {
    env::var("TERMINAL")
        .or_else(|_| env::var("COLORTERM"))
        .or_else(|_| env::var("TERM"))
        .unwrap_or("Unknown".to_string())
}

fn get_terminal_font() -> String {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "monospace-font-name"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if !output_str.trim().is_empty() {
            return output_str.trim().trim_matches('\'').to_string();
        }
    }
    if let Ok(output) = Command::new("konsole").arg("--list-profiles").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(profile) = output_str.lines().next() {
            if let Ok(font_output) = Command::new("konsoleprofile")
                .args(["Profile", profile, "-p", "Font"])
                .output()
            {
                let font_str = String::from_utf8_lossy(&font_output.stdout);
                if !font_str.trim().is_empty() {
                    return font_str.trim().to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_system_font() -> String {
    let desktop_session = env::var("DESKTOP_SESSION").unwrap_or_default().to_lowercase();
    if desktop_session.contains("gnome") {
        if let Ok(output) = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "font-name"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.trim().trim_matches('\'').to_string();
        }
    } else if desktop_session.contains("kde") {
        if let Ok(output) = Command::new("kreadconfig5")
            .args(["--group", "General", "--key", "font"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.trim().to_string();
        }
    }
    "Unknown".to_string()
}

fn print_color_strip() {
    let colors = [
        "\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m",
        "\x1b[97m", "\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m", "\x1b[97m",
    ];
    let strip: String = colors.iter().map(|color| format!("{}█", color)).collect();
    println!("{}\x1b[0m", strip);
}

fn get_distro_logo(distro_name: &str) -> String {
    if distro_name.contains("Zorin") {
        return r#"
        ██████████        
    ████████████████    
  ████████████████████  
████████████████████████
████████████████████████
████████████████████████
████████████████████████
  ████████████████████  
    ████████████████    
        ██████████        
"#.to_string();
    }
    "Unsupported distro for artwork display".to_string()
}

fn main() {
    let matches = ClapCommand::new("QuickFetch")
        .version("1.0")
        .about("System Information Tool")
        .arg(
            Arg::new("experimental")
                .long("experimental")
                .action(clap::ArgAction::SetTrue)
                .help("Display with artwork similar to Neofetch"),
        )
        .get_matches();

    let os_info = get_os_info();
    let kernel = get_kernel_info();
    let architecture = get_arch_info();
    let (cpu_name, cpu_cores) = get_cpu_info();
    let (total_memory, used_memory) = get_memory_info();
    let (swap_total, swap_used) = get_swap_info();
    let uptime = get_uptime_info();
    let hostname = get_hostname_info();
    let user = get_user_info();
    let gpu_info = get_gpu_info();
    let package_manager_info = get_package_manager_info();
    let resolution = get_resolution();
    let desktop_environment = get_desktop_environment();
    let window_manager = get_window_manager();
    let window_manager_theme = get_window_manager_theme();
    let gtk_theme = get_gtk_theme();
    let icon_theme = get_icon_theme();
    let terminal = get_terminal();
    let terminal_font = get_terminal_font();
    let system_font = get_system_font();
    let disk_usage_info = get_disk_usage_info();
    let local_ip = get_local_ip_info();
    let battery_info = get_battery_info();
    let locale_info = get_locale_info();

    if matches.get_flag("experimental") {
        let logo = get_distro_logo(&os_info);
        if logo == "Unsupported distro for artwork display" {
            println!("User: {}@{}", user, hostname);
            println!("OS: {}", os_info);
            println!("Kernel: {}", kernel);
            println!("Architecture: {}", architecture);
            println!("CPU: {} ({} cores)", cpu_name, cpu_cores);
            println!("GPU: {}", gpu_info);
            println!(
                "Memory: {:.2}GiB / {:.2}GiB",
                used_memory as f64 / (1024.0 * 1024.0 * 1024.0),
                total_memory as f64 / (1024.0 * 1024.0 * 1024.0)
            );
            println!(
                "Swap: {:.2}GiB / {:.2}GiB",
                swap_used as f64 / (1024.0 * 1024.0 * 1024.0),
                swap_total as f64 / (1024.0 * 1024.0 * 1024.0)
            );
            println!("Uptime: {}", uptime);
            println!("Resolution: {}", resolution);
            println!("DE: {}", desktop_environment);
            println!("WM: {}", window_manager);
            println!("WM Theme: {}", window_manager_theme);
            println!("Theme: {}", gtk_theme);
            println!("Icons: {}", icon_theme);
            println!("Terminal: {}", terminal);
            println!("Terminal Font: {}", terminal_font);
            println!("System Font: {}", system_font);
            for (mountpoint, total, used) in disk_usage_info {
                println!(
                    "Disk ({}): {:.2}GiB / {:.2}GiB",
                    mountpoint,
                    used as f64 / (1024.0 * 1024.0 * 1024.0),
                    total as f64 / (1024.0 * 1024.0 * 1024.0)
                );
            }
            println!("Local IP: {}", local_ip);
            println!("Battery: {}", battery_info);
            println!("Locale: {}", locale_info);
            for (manager, count) in package_manager_info {
                println!("{}: {} packages", manager.capitalize(), count);
            }
            print_color_strip();
        } else {
            println!("{}", logo);
            let mut details = format!(
                "User: {}@{}\n\
                 OS: {}\n\
                 Kernel: {}\n\
                 Architecture: {}\n\
                 CPU: {} ({} cores)\n\
                 GPU: {}\n\
                 Memory: {:.2}GiB / {:.2}GiB\n\
                 Swap: {:.2}GiB / {:.2}GiB\n\
                 Uptime: {}\n\
                 Resolution: {}\n\
                 DE: {}\n\
                 WM: {}\n\
                 WM Theme: {}\n\
                 Theme: {}\n\
                 Icons: {}\n\
                 Terminal: {}\n\
                 Terminal Font: {}\n\
                 System Font: {}\n\
                 Local IP: {}\n\
                 Battery: {}\n\
                 Locale: {}\n",
                user,
                hostname,
                os_info,
                kernel,
                architecture,
                cpu_name,
                cpu_cores,
                gpu_info,
                used_memory as f64 / (1024.0 * 1024.0 * 1024.0),
                total_memory as f64 / (1024.0 * 1024.0 * 1024.0),
                swap_used as f64 / (1024.0 * 1024.0 * 1024.0),
                swap_total as f64 / (1024.0 * 1024.0 * 1024.0),
                uptime,
                resolution,
                desktop_environment,
                window_manager,
                window_manager_theme,
                gtk_theme,
                icon_theme,
                terminal,
                terminal_font,
                system_font,
                local_ip,
                battery_info,
                locale_info
            );
            for (mountpoint, total, used) in disk_usage_info {
                details += &format!(
                    "Disk ({}): {:.2}GiB / {:.2}GiB\n",
                    mountpoint,
                    used as f64 / (1024.0 * 1024.0 * 1024.0),
                    total as f64 / (1024.0 * 1024.0 * 1024.0)
                );
            }
            for (manager, count) in package_manager_info {
                details += &format!("{}: {} packages\n", manager.capitalize(), count);
            }
            println!("{}", details);
            print_color_strip();
        }
    } else {
        println!("User: {}@{}", user, hostname);
        println!("OS: {}", os_info);
        println!("Kernel: {}", kernel);
        println!("Architecture: {}", architecture);
        println!("CPU: {} ({} cores)", cpu_name, cpu_cores);
        println!("GPU: {}", gpu_info);
        println!(
            "Memory: {:.2}GiB / {:.2}GiB",
            used_memory as f64 / (1024.0 * 1024.0 * 1024.0),
            total_memory as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        println!(
            "Swap: {:.2}GiB / {:.2}GiB",
            swap_used as f64 / (1024.0 * 1024.0 * 1024.0),
            swap_total as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        println!("Uptime: {}", uptime);
        println!("Resolution: {}", resolution);
        println!("DE: {}", desktop_environment);
        println!("WM: {}", window_manager);
        println!("WM Theme: {}", window_manager_theme);
        println!("Theme: {}", gtk_theme);
        println!("Icons: {}", icon_theme);
        println!("Terminal: {}", terminal);
        println!("Terminal Font: {}", terminal_font);
        println!("System Font: {}", system_font);
        for (mountpoint, total, used) in disk_usage_info {
            println!(
                "Disk ({}): {:.2}GiB / {:.2}GiB",
                mountpoint,
                used as f64 / (1024.0 * 1024.0 * 1024.0),
                total as f64 / (1024.0 * 1024.0 * 1024.0)
            );
        }
        println!("Local IP: {}", local_ip);
        println!("Battery: {}", battery_info);
        println!("Locale: {}", locale_info);
        for (manager, count) in package_manager_info {
            println!("{}: {} packages", manager.capitalize(), count);
        }
        print_color_strip();
    }
}

trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for String {
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}