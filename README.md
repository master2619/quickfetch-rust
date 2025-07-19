 **QuickFetch-rust**  

QuickFetch is a lightweight system information tool designed for Linux systems. It provides concise and visually appealing details about your system's configuration and usage. With QuickFetch, you can quickly access essential information such as the operating system, kernel version, hardware architecture, CPU and GPU specifications, memory usage, uptime, and package manager statistics.  

## **Features**  
✔ Displays essential system information (OS, kernel, CPU, GPU, memory, uptime, etc.)  
✔ Supports multiple display resolutions and desktop environments  
✔ Detects and displays disk usage across multiple partitions  
✔ Shows battery status and local IP  
✔ Package Manager Detection (Only on Debian-based systems: `dpkg`, `apt`, `snap`, `flatpak`)  
✔ Works on all major Linux distributions  
✔ Lightweight and easy to install  

### **Sample Output**  
```
User: hypr@HP-Linux
OS: Manjaro Linux
Kernel: 6.12.12-2-MANJARO
Architecture: x86_64
CPU: AMD Ryzen 3 3250U with Radeon Graphics (4 cores)
GPU: No GPU found
Memory: 2.81GiB / 21.45GiB
Swap: 0.00GiB / 23.59GiB
Uptime: 3:01:59
Resolution: 1920x1080
DE: Hyprland
WM: Wayland
WM Theme: Unknown
Theme: Rose-Pine
Icons: Tela-circle-pink
Terminal: truecolor
Terminal Font: CaskaydiaCove Nerd Font Mono 9
System Font: Unknown
Disk (/): 80.18GiB / 907.62GiB
Disk (/home): 80.18GiB / 907.62GiB
Disk (/var/cache): 80.18GiB / 907.62GiB
Disk (/var/log): 80.18GiB / 907.62GiB
Disk (/boot/efi): 0.00GiB / 0.29GiB
Disk (/run/media/deepesh/DATA): 271.19GiB / 931.50GiB
Local IP: 192.168.1.16
Battery: 24.196787148594378% [Discharging]
Locale: en_IN
```
> **Note:** Debian-based Linux systems may also display package manager stats.
---

## **Installation**  

### **Easy Installation (Debian-Based Distros)**  
For Debian-based distributions (Ubuntu, Zorin OS, Pop!_OS, Linux Mint, etc.), you can install QuickFetch using a single command:  
```bash
sudo apt install curl && curl -sSL https://github.com/master2619/quickfetch-rust/releases/download/release/installer.sh | sudo sh
```
### **Easy Installation (Fedora-based Distros)**
For Fedora-based Linux distributions.
```bash
sudo dnf install curl -y && curl -sSL https://github.com/master2619/quickfetch-rust/releases/download/release/installer.sh | sudo sh
```
### **Easy Installation (Arch-Based Distros)**
For Arch-based distributions like Manjaro, Arco Linux, etc.
```bash
sudo pacman -S curl && curl -sSL https://github.com/master2619/quickfetch-rust/releases/download/release/installer.sh | sudo sh
```

## **Usage**  
```bash
quickfetch
```

---

## **License**  
GPL 3.0 License. See the LICENSE file for details.  

---

## **Contributing**  
Fork the repository and create a pull request with your changes.  

---

## **Issues & Troubleshooting**  
🔹 Ensure the binary is in `/usr/bin/`.  
🔹 Open an **issue** on GitHub for help.  

---

## **Acknowledgements**  
- **Inspired by Neofetch**  
