

# Project Name: Chat Interaction Recorder

## Description

Chat Interaction Recorder is a Rust-based CLI tool designed to capture, process, and manage interactions from chat applications or services. Utilizing asynchronous Rust features and terminal interaction, this tool streams chat responses, processes them in real-time, and provides functionalities such as displaying interactions in the terminal with custom colors and copying selected interactions to the clipboard.

The tool is built with a focus on performance and user experience, offering a responsive and efficient way to handle chat data. Features include real-time processing of streamed chat data, color-coded terminal output for enhanced readability, and the ability to copy interactions directly to the clipboard for easy sharing and documentation.

## Features

- **Real-Time Chat Data Processing**: Stream chat data in real-time and process incoming messages without delay.
- **Color-Coded Output**: Customize terminal output with color coding for better readability and differentiation of chat interactions.
- **Clipboard Support**: Copy selected chat interactions directly to the clipboard, allowing for easy sharing and documentation.
- **Flexible Interaction Management**: Store and manage chat interactions with support for retrieving and displaying historical data.
- **Asynchronous Rust**: Leverage asynchronous Rust for efficient handling of I/O-bound tasks and concurrent data processing.

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- Terminal or command prompt that supports ANSI escape codes for colored output

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/chat-interaction-recorder.git
   ```

2. Navigate to the project directory:

   ```sh
   cd chat-interaction-recorder
   ```

3. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

4. The executable will be located in `target/release/`.

### Usage

To start capturing and processing chat interactions, run the following command in your terminal:

```sh
cargo run
```

#### Commands

- `/copy [number]`: Copy the last `[number]` interactions to the clipboard. If `[number]` is omitted, the most recent interaction is copied.
- `/exit`: Exit the application.

## Development

To contribute to Chat Interaction Recorder or customize it for your needs, follow the standard Rust development practices:

- Make changes or add features in a separate branch.
- Test your changes thoroughly.
- Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Special thanks to all contributors and users of the project.
- This project was inspired by the need for efficient real-time processing of chat data.

