use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::f32::consts::PI;
use std::io::BufWriter;

// Funkcja jądra Lanczosa - https://www.youtube.com/watch?v=ijmd6XyG2HA
fn lanczos_kernel(x: f32, a: f32) -> f32 {
    if x == 0.0 {
        1.0
    } else if x > -a && x < a {
        a * (PI * x).sin() * (PI * x / a).sin() / (PI * PI * x * x)
    } else {
        0.0
    }
}

fn read_asc_file(file_path: &str) -> io::Result<(Vec<Vec<f32>>, usize, usize, f32, f32)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut lines = reader.lines();
    let mut ncols = 0;
    let mut nrows = 0;
    let mut heightmap = Vec::new();
    let mut dx = 0.0;
    let mut dy = 0.0;

    //rust documentation
    for _ in 0..6 {
        if let Some(line) = lines.next() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.get(0) {
                Some(&"ncols") => ncols = parts.get(1).unwrap_or(&"0").parse().unwrap(),
                Some(&"nrows") => nrows = parts.get(1).unwrap_or(&"0").parse().unwrap(),
                Some(&"dx") => dx = parts.get(1).unwrap_or(&"0").parse().unwrap(),
                Some(&"dy") => dy = parts.get(1).unwrap_or(&"0").parse().unwrap(),
                _ => {}
            }
        }
    }

    for line in lines {
        let line = line?;
        let row: Vec<f32> = line.split_whitespace()
                                .map(|s| s.parse().unwrap_or(0.0))
                                .collect();
        if row.len() == ncols {
            heightmap.push(row);
        }
    }

    Ok((heightmap, ncols, nrows, dx, dy))
}

fn write_obj_file(file_path: &str, heightmap: Vec<Vec<f32>>, ncols: usize, nrows: usize, dx: f32, dy: f32) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file); // Buforowanie ChatGPT
    writeln!(writer, "# OBJ file generated from .asc")?;

    for i in 0..nrows {
        for j in 0..ncols {
            let x = j as f32 * dx;
            let y = heightmap[i][j];
            let z = i as f32 * dy;
            writeln!(writer, "v {} {} {}", x, y, z)?;
        }
    }

    for i in 0..nrows-1 {
        for j in 0..ncols-1 {
            let top_left = i * ncols + j + 1;
            let top_right = top_left + 1;
            let bottom_left = (i + 1) * ncols + j + 1;
            let bottom_right = bottom_left + 1;

            writeln!(writer, "f {} {} {}", top_left, bottom_left, top_right)?;
            writeln!(writer, "f {} {} {}", top_right, bottom_left, bottom_right)?;
        }
    }

    Ok(())
}

//pomoc Copilot'a przy tworzeniu algorytmu
fn interpolate_heightmap_lanczos(heightmap: Vec<Vec<f32>>, ncols: usize, nrows: usize, _dx: f32, _dy: f32, a: f32) -> Vec<Vec<f32>> {
    let new_ncols = (ncols - 1) * 2 + 1;
    let new_nrows = (nrows - 1) * 2 + 1;
    let mut interpolated_heightmap = vec![vec![0.0; new_ncols]; new_nrows];

    for i in 0..new_nrows {
        for j in 0..new_ncols {
            let x = j as f32 / 2.0;
            let y = i as f32 / 2.0;

            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for ii in (y.floor() as isize - 2).max(0)..=(y.ceil() as isize + 2).min(nrows as isize - 1) {
                for jj in (x.floor() as isize - 2).max(0)..=(x.ceil() as isize + 2).min(ncols as isize - 1) {
                    let weight = lanczos_kernel(y - ii as f32, a) * lanczos_kernel(x - jj as f32, a);
                    sum += heightmap[ii as usize][jj as usize] * weight;
                    weight_sum += weight;
                }
            }

            interpolated_heightmap[i][j] = sum / weight_sum;
        }
    }

    interpolated_heightmap
}

fn write_height_file(file_path: &str, min_height: f32, max_height: f32) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    writeln!(file, "Minimum height: {}", min_height)?;
    writeln!(file, "Maxiumim height: {}", max_height)?;
    Ok(())
}

fn main() {
    let input_path = "result2.asc";
    let output_path = "model2.obj";
    let height_path = "height.txt";

    match read_asc_file(input_path) {
        Ok((heightmap, ncols, nrows, dx, dy)) => {
            let interpolated_heightmap = interpolate_heightmap_lanczos(heightmap, ncols, nrows, dx / 2.0, dy / 2.0, 3.0);

            let min_height = interpolated_heightmap.iter().flat_map(|row| row.iter()).cloned().fold(f32::INFINITY, f32::min); //ChatGPT
            let max_height = interpolated_heightmap.iter().flat_map(|row| row.iter()).cloned().fold(f32::NEG_INFINITY, f32::max); //ChatGPT

            if let Err(e) = write_obj_file(output_path, interpolated_heightmap, (ncols - 1) * 2 + 1, (nrows - 1) * 2 + 1, dx / 2.0, dy / 2.0) {
                println!("Błąd zapisu pliku: {}", e);
            } else {
                println!("Interpolowana siatka 3D została zapisana do pliku .obj!");
            }

            if let Err(e) = write_height_file(height_path, min_height, max_height) {
                println!("Błąd zapisu pliku: {}", e);
            } else {
                println!("Plik z wysokościami został zapisany do height.txt!");
            }
        },
        Err(e) => println!("Błąd odczytu pliku: {}", e),
    }

    let file_path = "model2.obj";
    if let Err(e) = opener::open(file_path) {
        eprintln!("Nie udało się otworzyć pliku: {}", e);
    }
}
