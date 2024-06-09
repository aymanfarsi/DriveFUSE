# DriveFUSE

<p align="center">
  <a href="https://github.com/aymanfarsi/DriveFUSE"><img src="assets/drivefuse.png" alt="DriveFUSE" height="120" /></a>
</p>

<p align="center">
  <strong>DriveFUSE is a cross-platform cloud storage mounter that allows you to mount your cloud storage as a local drive on your computer.</strong>
</p>

## Contents
- [DriveFUSE](#drivefuse)
  - [Contents](#contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Building from source](#building-from-source)
  - [Contributing](#contributing)
  - [Credits](#credits)
  - [License](#license)

## Features

- **Cross-platform**: DriveFUSE is built using Rust, which allows it to run on Windows, macOS, and Linux.
- **Cloud storage support**: DriveFUSE supports a wide range of cloud storage providers, including Google Drive, Dropbox, OneDrive, and more.
- **Mounting**: DriveFUSE allows you to mount your cloud storage as a local drive on your computer, making it easy to access and manage your files.

## Installation

To use DriveFUSE, you need to have the following dependencies installed on your system:

> Auto-installing dependencies when missing is a feature that will be added in the future.

- You need to install `rclone` following the instructions on their [website](https://rclone.org/install/).
- On Windows, you need to have `WinFsp`.
- On Unix systems (Linux and macOS), you need some variation of `fusermount3` installed.

## Building from source

In order to build DriveFUSE from source, you need to have the Rust programming language installed on your system. Use this command to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. Clone the repository:

```bash
git clone https://github.com/aymanfarsi/DriveFUSE.git
```

2. Change into the project directory and rename it to avoid issues with building the executable:

```bash
mv DriveFUSE drive_fuse
cd drive_fuse
```

3. Build the project:

```bash
cargo build --release
```

4. The binary will be located in the `target/release` directory. You can run it using:

```bash
./target/release/drive_fuse
```

5. You can also install the binary to your system using:

```bash
cargo install --path .
```

6. You can now run the binary using `drive_fuse` or search for it in your system's application menu:

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request if you have any ideas, bug reports, or feature requests.

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your changes.
3. Make your changes and commit them.
4. Push the changes.
5. Submit a pull request.

## Credits

DriveFUSE is built and relies on the following technologies:

- [Rust](https://www.rust-lang.org/): A systems programming language that focuses on safety, speed, and concurrency.
- [egui](https://www.github.com/emilk/egui): A simple, fast, and highly portable immediate mode GUI library.
- [rclone](https://rclone.org/): A command-line program to manage files on cloud storage.
- [WinFsp](https://winfsp.dev/): A Windows File System Proxy that allows you to create user mode file systems.
- [fusermount3](https://www.man7.org/linux/man-pages/man1/fusermount3.1.html): A program to mount and unmount FUSE filesystems.

## License

DriveFUSE is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
