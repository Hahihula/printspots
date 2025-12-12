# PrintSpots

PrintSpots is a free and open-source desktop application that allows you to create multi-color and grayscale 3D prints from bitmap images on any standard 3D printer. It generates relief-based artworks by intelligently layering filaments to produce the final object.

## About The Project

This application provides a complete workflow for turning your images into 3D printable objects. The key feature of PrintSpots is its ability to create multicolor prints without requiring a multimaterial printer. This is achieved by a process that involves a single filament change per color.

The application guides you through a calibration process to create a specific profile for your printer and filaments, ensuring accurate color prediction.

### Built With

*   [Tauri](https://tauri.app/)
*   [Vue.js](https://vuejs.org/)
*   [Rust](https://www.rust-lang.org/)

## Installation

Binary builds are available for Linux, macOS, and Windows. You only need to build from source if you are a developer.

*   **All builds:** [PrintSpots Webpage](https://printspots.org)
*   **Linux:** [GitLab Releases](https://gitlab.com/printspots/printspots/-/releases)
*   **macOS and Windows:** [GitHub Releases](https://github.com/hahihula/printspots/releases)

## Workflow

The application follows a simple, step-by-step workflow:

1.  **Initial Setup:** On the first run, you will be guided through creating a printer and palette configuration. This will generate a 3D printable calibration object.
2.  **Calibration:**
    *   Print the calibration object.
    *   Scan or photograph the printed object.
    *   Use the in-app tool to analyze the image and create a palette configuration file specific to your printer and filaments.
3.  **Project Creation:**
    *   Start a new project and load a bitmap image.
    *   Use the built-in image editor to tweak the image.
    *   Generate a prediction of the final print.
    *   If you are satisfied with the prediction, generate a `3mf` printable file.

## The Printing Process

The generated `3mf` files are designed to be printed on a standard FDM printer with manual filament changes. For a grayscale print, the process is as follows:

1.  Start the print with black filament.
2.  The printer will pause at a specific layer height, and you will be prompted to change the filament to white.
3.  Resume the print to complete the object.

## For Developers

If you want to contribute to the project, you will need to build the application from source.

### Prerequisites

You need to have Node.js, Yarn, and Rust installed on your system.

*   **Node.js and Yarn:**
    We recommend using a version manager like `nvm` or `fnm` to install Node.js and npm. Then, install Yarn:
    ```sh
    npm install -g yarn
    ```

*   **Rust:**
    Install Rust using `rustup`:
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

*   **Tauri Prerequisites:**
    Follow the official Tauri guide to set up your environment for Tauri development:
    [https://tauri.app/v1/guides/getting-started/prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1.  Clone the repo:
    ```sh
    git clone https://github.com/your_username/printspots.git
    ```
2.  Navigate to the `gui` directory:
    ```sh
    cd printspots/gui
    ```
3.  Install NPM packages:
    ```sh
    yarn
    ```

### Development

To run the application in development mode, use the following command. This will open the app in a new window with hot-reloading enabled.

```sh
yarn tauri dev
```

### Building for Production

To build the application for production, use the following command. The executable will be located in the `src-tauri/target/release` directory.

```sh
yarn tauri build
```

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

## Project Structure

The project is organized into two main parts: the frontend Vue.js application and the backend Rust application.

*   `src/`: Contains the Vue.js frontend code.
    *   `components/`: Reusable Vue components.
    *   `views/`: Application pages.
    *   `stores/`: Pinia stores for state management.
    *   `router.js`: Vue Router configuration.
    *   `main.js`: The entry point for the Vue application.
*   `src-tauri/`: Contains the Rust backend code.
    *   `src/main.rs`: The entry point for the Rust application.
    *   `src/commands.rs`: Custom Tauri commands.
    *   `tauri.conf.json`: Tauri configuration file.
*   `public/`: Static assets.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more information.
