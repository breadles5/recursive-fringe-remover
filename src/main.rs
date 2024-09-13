use std::path::{Path, PathBuf};
use std::time::Instant;
use image::{GenericImageView, ImageBuffer, Rgba};
use walkdir::WalkDir;
use rfd::FileDialog;
use rayon::prelude::*;

fn main() {
    // Open a file explorer window to select the directory
    let dir_to_process = FileDialog::new()
        .set_directory(".")
        .pick_folder();

    let dir_path = match dir_to_process {
        Some(path) => path,
        None => {
            eprintln!("No directory selected.");
            std::process::exit(1);
        }
    };

    // Collect all PNG files in the directory and its subdirectories
    let png_files: Vec<PathBuf> = WalkDir::new(&dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
        .map(|e| e.path().to_owned())
        .collect();

    let total_files = png_files.len();
    println!("Found {} PNG files to process.", total_files);

    let start_time = Instant::now();

    // Process files in parallel
    png_files.par_iter().for_each(|file_path| {
        println!("Processing image: {:?}", file_path);
        match process_image(file_path) {
            Ok(size) => println!("Finished processing {:?}. Output size: {}x{}", file_path, size.0, size.1),
            Err(e) => eprintln!("Error processing {:?}: {}", file_path, e),
        }
    });

    let total_time = start_time.elapsed();
    println!("All images processed. Total time: {:?}", total_time);
}

fn process_image(file_path: &Path) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let img = image::open(file_path)?;
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            if pixel[3] == 0 {
                output.put_pixel(x, y, compute_color(x, y, &img));
            } else {
                output.put_pixel(x, y, pixel);
            }
        }
    }

    output.save(file_path)?;
    Ok((width, height))
}

fn compute_color(x: u32, y: u32, img: &image::DynamicImage) -> Rgba<u8> {
    let mut r_sum = 0u32;
    let mut g_sum = 0u32;
    let mut b_sum = 0u32;
    let mut a_sum = 0u32;
    let mut count = 0u32;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && nx < img.width() as i32 && ny < img.height() as i32 {
                let pixel = img.get_pixel(nx as u32, ny as u32);
                if pixel[3] != 0 {
                    r_sum += pixel[0] as u32 * pixel[3] as u32;
                    g_sum += pixel[1] as u32 * pixel[3] as u32;
                    b_sum += pixel[2] as u32 * pixel[3] as u32;
                    a_sum += pixel[3] as u32;
                    count += 1;
                }
            }
        }
    }

    if count > 0 {
        Rgba([
            (r_sum / a_sum) as u8,
            (g_sum / a_sum) as u8,
            (b_sum / a_sum) as u8,
            0,
        ])
    } else {
        Rgba([0, 0, 0, 0])
    }
}
