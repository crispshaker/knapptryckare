# knapptryckare

A simple Rust application for controlling Switchbot bot devices via Bluetooth.

## Prerequisites

Before you proceed, ensure that you have the following prerequisites installed on your system:

- Rust and Cargo: You can download and install Rust and Cargo by following the official installation guide at [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Building

1. **Clone the Repository**:

    If you haven't already, clone the repository containing the project to your local machine.

    ```bash
    git clone https://github.com/crispshaker/knapptryckare.git
    cd knapptryckare
    ```

2. **Build the Program**:

    ```bash
    cargo build --release
    ```

## Creating an Executable

Once you have successfully built the project, you can create an executable for *knapptryckare* by following these steps:

1. **Locate the Executable**:

    After building the project, the compiled executable file will be located in the `target/release` directory within your project folder. The file name will be the same as your project name. In this case, it will be named *knapptryckare*.

2. **Copy the Executable**:

    Copy the executable to a location of your choice or distribute it as needed. You can also rename the executable to make it more meaningful for your use case.

    ```bash
    cp target/release/knapptryckare /path/to/destination/
    ```

3. **Make the Executable Executable**:

    If the copied file is not already marked as executable, you can do so using the `chmod` command.

    ```bash
    chmod +x /path/to/destination/
    ```

## Run executable

In order to run the project successfully, you need to provide the Bluetooth MAC address as an argument. Here's the correct format:

    knapptryckare 00:00:00:00:00:00
    
Replace 00:00:00:00:00:00 with the actual Bluetooth MAC address you want to use.

## Tested Environment

This project has been thoroughly tested and confirmed to work under Linux Debian. It should work on other Linux distributions as well.

## Disclaimer

USE THIS PROJECT AT YOUR OWN RISK. The authors and contributors of this project make no guarantees or warranties, and assume no liability for any consequences resulting from the use of this software. Be cautious when using this project, especially in production environments, and ensure that you have appropriate backups and safeguards in place.


## License

This project is open-source software and is distributed under the terms of the MIT License. You can find the license details in the `LICENSE` file in the project repository.