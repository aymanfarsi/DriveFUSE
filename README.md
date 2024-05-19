# Rclone App

This is a Rust application that provides a user interface for managing and interacting with rclone configurations.

## Features

- **Storage Management**: The application allows you to add, edit, and remove storages. Each storage is represented by a `Storage` object, which is parsed from the rclone configuration file.

- **Backup and Restore**: The application provides functionality to create backups of your rclone configuration and restore them when needed.

- **Configuration Reading and Writing**: The application can read and write to the rclone configuration file.

## Tools Used

- **Rust**: The application is written in Rust, a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

- **Rclone**: This application is a user interface for rclone, a command line program to manage files on cloud storage.

- **Cargo**: Cargo is the Rust package manager. It is used for managing Rust dependencies, building the project, and more.

- **IDEA**: The project is developed using the IDEA IDE.

## Building the Project

To build the project, run the `build_release` script.

## Running the Project

To run the project, navigate to the `target/release` directory and run the compiled binary.

## Contributing

Contributions are welcome. Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the terms of the MIT license.