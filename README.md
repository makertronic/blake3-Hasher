# Blake3 Hasher

This project is a command-line tool written in Rust to compute the Blake3 hashes of all files within a specified directory.

## Installation

You'll need [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your machine to compile and run this project.

You can use the compiled version in the release directory, or you can compile it : 

1. Install Rust and Cargo:

    Follow the instructions on the official Rust page [here](https://www.rust-lang.org/tools/install).

2. Clone this repository:

    ```bash
    git clone https://github.com/makertronic/Blake3Hasher.git
    ```

    (Replace "username" with your GitHub username.)

3. Navigate to the project folder:

    ```bash
    cd Blake3Hasher
    ```


## Usage

To use this tool, build the project and run the executable:

1. Build the project:

    ```bash
    cargo build --release
    ```

2. Run the tool:

    ```bash
    cargo run /path/to/directory
    ```

    (Replace "/path/to/directory" with the path of the directory you want to hash the files of.)

## Contributing

Contributions to this project are welcome! If you have a feature request or a bug report, please open an issue. If you have a fix or a new feature to submit, please open a pull request.

## License

This project is distributed under the MIT license. See the `LICENSE` file for more details.
