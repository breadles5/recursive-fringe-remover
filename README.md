# recursive fringe remover

A command-line tool to remove fringes from images recursively.   
my take on the [original gui based fringe remover](https://github.com/RoanH/FringeRemover)

## Installation

Download the latest release from the [Releases](https://github.com/breadles5/recursive-fringe-remover/releases) page.

Alternatively, if you prefer to build from source:

1. Ensure you have Rust installed on your system. If not, you can install it from https://www.rust-lang.org/.

2. Clone this repository:
   ```
   git clone https://github.com/breadles5/recursive-fringe-remover.git
   cd recursive-fringe-remover
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Run the program if built from source:
   ```
   cargo run --release
   ```
   to run from [Releases](https://github.com/breadles5/recursive-fringe-remover/releases):
double click the executable file in file explorer.  
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
- Processing time depends on the number and size of images, as well as your system's capabilities