use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use citygml_spatial_id::theme::building::parser::BuildingParser;
use kasane_logic::{SetOnMemory, Solid};

fn main() {
    // 1. 設定
    let input_path =
        "data/11234_yashio-shi_pref_2023_citygml_2_op/udx/bldg/53395655_bldg_6697_op.gml";
    let output_path = "building_spatial_ids.txt";
    let zoom_level: u8 = 25; // 空間IDのズームレベル（適宜変更してください）
    let epsilon = 0.01; // 許容誤差 1cm
    let target_count = 100; // 処理する件数

    if !Path::new(input_path).exists() {
        eprintln!("Error: File not found at {}", input_path);
    }

    // 2. ファイルとパーサーの準備
    let file = File::open(input_path).unwrap();
    let parser = BuildingParser::new(file);
    let mut writer = BufWriter::new(File::create(output_path).unwrap());

    println!("Starting processing: target {} buildings...", target_count);
    println!("Output will be written to: {}", output_path);

    let mut set = SetOnMemory::new();

    let mut processed_count = 0;
    let mut success_count = 0;

    // 3. ループ処理 (takeで100件に制限)
    for building in parser.take(target_count) {
        processed_count += 1;

        // gml:id を取得
        let gml_id = &building.gml_id;

        // Solid の作成（閉合性チェック含む）
        // building.surfaces は Vec<Vec<Coordinate>>
        match Solid::new(building.surfaces.clone(), epsilon) {
            Ok(solid) => {
                // 空間ID (SingleId) の生成
                match solid.single_ids(zoom_level) {
                    Ok(ids) => {
                        for id in ids {
                            unsafe { set.join_insert_unchecked(id) };
                        }
                    }
                    Err(e) => {
                        // ID生成失敗（ズームレベル範囲外など）
                        eprintln!("[{}] ID Gen Error: {}", gml_id, e);
                        writeln!(writer, "{}\tERROR_ID_GEN: {}", gml_id, e).unwrap();
                    }
                }
            }
            Err(e) => {
                // Solid作成失敗（閉じていない、頂点不足など）
                // エラー内容をファイルにも記録しておくと後で分析しやすい
                eprintln!("[{}] Solid Error: {}", gml_id, e);
                writeln!(writer, "{}\tERROR_SOLID: {}", gml_id, e).unwrap();
            }
        }
    }

    let mut file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(file);

    for ele in set.range_ids() {
        write!(writer, "{},", ele).unwrap();
    }

    writer.flush().unwrap();
}
