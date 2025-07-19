# QuickFetch

QuickFetch is a lightweight system information tool designed for Linux systems. It provides concise and visually appealing details about your system's configuration and usage. With QuickFetch, you can quickly access essential information such as the operating system, kernel version, hardware architecture, CPU and GPU specifications, memory usage, uptime, and package manager statistics.

## Features

### Sample Output

```bash
User: deepesh@HP-Linux-Laptop
OS: Zorin OS 17.1
Kernel: 6.9.3-x64v3-xanmod1
Architecture: x86_64
CPU: AMD Ryzen 3 3250U with Radeon Graphics (4 cores)
GPU: Picasso/Raven 2 [Radeon Vega Series / Radeon Vega Mobile Series]
Memory: 2.25GiB / 21.45GiB
Uptime: 4:27:21
Resolution: XWAYLAND0
DE: Zorin
WM: Wayland
WM Theme: 'Adwaita'
Theme: 'ZorinBlue-Dark'
Icons: 'ZorinBlue-Dark'
Terminal: truecolor
Dpkg: 2813 packages
Apt: 2809 packages
Snap: 7 packages
Flatpak: 39 packages
```

### Installation

#### Easy Installation

To install QuickFetch with a single command:

```bash
sudo apt install curl && curl -sSL https://github.com/master2619/quickfetch/releases/download/release-2/installer.sh | sudo sh
```

#### Manual Compilation

QuickFetch requires the following Python libraries:

- psutil
- distro
- colorama
- GPUtil

Install the dependencies using pip:

```bash
pip3 install psutil distro colorama GPUtil
```

To compile QuickFetch into a standalone executable using PyInstaller:

```bash
pip3 install pyinstaller
```

Ensure PyInstaller is in your PATH:

```bash
export PATH=$PATH:/home/$USER/.local/bin
```

Compile the script:

```bash
pyinstaller --onefile quickfetch.py
```

Move the compiled binary to `/usr/bin` for permanent installation:

```bash
sudo mv /home/$USER/quickfetch/dist/quickfetch /usr/bin/quickfetch
```

### Usage

Simply run the `quickfetch` executable from your terminal:

```bash
quickfetch
```

### Making QuickFetch Accessible Everywhere

To make the `quickfetch` binary accessible from anywhere in the terminal, add the parent directory of the binary file to your `~/.bashrc` or `~/.profile`:

```bash
echo 'export PATH=$PATH:/home/$USER/Downloads/' >> ~/.bashrc
```

### License

This project is licensed under the GPL 3.0 License. See the [LICENSE](LICENSE) file for details.

### Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

### Issues

If you encounter any issues or have suggestions for improvements, please open an issue on the [GitHub repository](https://github.com/master2619/quickfetch).

## Acknowledgements

- Inspired by Neofetch
- Uses psutil for system information
- Uses distro for Linux distribution detection
- Uses GPUtil for GPU information
- Uses colorama for colored terminal output
