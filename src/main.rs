use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;

use citygml_spatial_id::theme::building::parser::BuildingParser;
use kasane_logic::{Polygon, Solid};
fn main() {
    let dir = Path::new("data/13116_toshima-ku_pref_2023_citygml_2_op/udx/bldg");

    // OBJ出力用ディレクトリの作成
    let output_dir = Path::new("output_obj");
    if !output_dir.exists() {
        fs::create_dir(output_dir).expect("failed to create output directory");
    }

    let files: Vec<PathBuf> = fs::read_dir(dir)
        .expect("failed to read bldg directory")
        .filter_map(|e| {
            let path = e.ok()?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("gml") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let total_files = files.len();
    let processed_files = Arc::new(AtomicUsize::new(0));

    // カウンタ
    let total_buildings = Arc::new(AtomicUsize::new(0));
    let total_surfaces = Arc::new(AtomicUsize::new(0));
    let invalid_surfaces = Arc::new(AtomicUsize::new(0));
    let valid_surfaces_buildings = Arc::new(AtomicUsize::new(0));
    let invalid_surfaces_buildings = Arc::new(AtomicUsize::new(0));
    let valid_solids = Arc::new(AtomicUsize::new(0));
    let invalid_solids = Arc::new(AtomicUsize::new(0));

    // OBJ出力数制限用カウンタ
    let exported_count = Arc::new(AtomicUsize::new(0));
    let max_export = 100;

    println!("Start verifying {} files...", total_files);

    files.par_iter().for_each(|path| {
        let file = File::open(path).unwrap();
        let parser = BuildingParser::new(file);

        for building in parser {
            total_buildings.fetch_add(1, Ordering::Relaxed);

            let mut building_surfaces = Vec::new();
            let mut has_invalid_surface = false;

            for points in building.surfaces {
                total_surfaces.fetch_add(1, Ordering::Relaxed);
                match Polygon::new(points) {
                    Ok(surface) => building_surfaces.push(surface),
                    Err(_) => {
                        invalid_surfaces.fetch_add(1, Ordering::Relaxed);
                        has_invalid_surface = true;
                    }
                }
            }

            if has_invalid_surface {
                invalid_surfaces_buildings.fetch_add(1, Ordering::Relaxed);
                continue;
            }

            valid_surfaces_buildings.fetch_add(1, Ordering::Relaxed);

            // Solid化の検証
            match Solid::new(building_surfaces) {
                Ok(solid) => {
                    valid_solids.fetch_add(1, Ordering::Relaxed);
                }
                Err(_) => {
                    invalid_solids.fetch_add(1, Ordering::Relaxed);
                }
            }
        }

        let done = processed_files.fetch_add(1, Ordering::Relaxed) + 1;
        if done % 10 == 0 || done == total_files {
            let percent = done as f64 / total_files as f64 * 100.0;
            eprintln!(
                "[PROGRESS] {}/{} files ({:.1}%)",
                done, total_files, percent
            );
        }
    });

    println!("========================================");
    println!("VERIFICATION RESULT");
    println!("========================================");
    println!(
        "Total Buildings Checked: {}",
        total_buildings.load(Ordering::Relaxed)
    );
    println!("Surface Check:");
    println!(
        "  Total Surfaces: {}",
        total_surfaces.load(Ordering::Relaxed)
    );
    println!(
        "  Invalid Surfaces: {}",
        invalid_surfaces.load(Ordering::Relaxed)
    );
    println!(
        "  Buildings with Invalid Surfaces: {}",
        invalid_surfaces_buildings.load(Ordering::Relaxed)
    );
    println!(
        "  Buildings with All Valid Surfaces: {}",
        valid_surfaces_buildings.load(Ordering::Relaxed)
    );
    println!("Solid Check:");
    println!(
        "閉じている建物の数: {}",
        valid_solids.load(Ordering::Relaxed)
    );
    println!(
        "壊れている建物の数: {}",
        invalid_solids.load(Ordering::Relaxed)
    );
    println!("----------------------------------------");
    println!(
        "Exported OBJs: {} (in ./output_obj/)",
        exported_count.load(Ordering::Relaxed)
    );
    println!("========================================");
}
