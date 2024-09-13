# recursive fringe remover

A command-line tool to remove fringes from images recursively.   
my take on [the oringal gui based fringe remover](https://github.com/RoanH/FringeRemover)

## Installation

1. Ensure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:
   ```
   git clone https://github.com/yourusername/recursive-fringe-remover.git
   cd recursive-fringe-remover
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Run the program:
   ```
   cargo run --release
   ```

2. A file dialog will open. Select the directory containing the PNG images you want to process.

3. The program will recursively search for PNG files in the selected directory and its subdirectories.

4. Each PNG file will be processed to remove fringes. The original files will be overwritten with the processed versions.

5. Progress and any errors will be displayed in the console.

## Features

- Recursively processes all PNG files in the selected directory and its subdirectories.
- Removes fringes by filling transparent pixels with colors computed from neighboring non-transparent pixels.
- Utilizes parallel processing for improved performance on multi-core systems.
- Provides progress updates and error reporting in the console.

## Notes

- This tool modifies images in-place. It's recommended to make a backup of your images before processing.
- Processing time depends on the number and size of images, as well as your system's capabilities.

## Requirements

- Rust 2021 edition or later
- Dependencies (automatically managed by Cargo):
  - image = "0.25.2"
  - walkdir = "2.3.3"
  - rfd = "0.14.1"
  - rayon = "1.7.0"
  - indicatif = "0.17.3"
