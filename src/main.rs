use citygml_spatial_id::theme::brid::xml::BuildingParser;
use kasane_logic::{Polygon, SetOnMemory, Solid};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;
use std::io::Write;
use std::time::Instant;
use std::{fs::File, io::BufWriter};

fn main() {
    let target = File::open(
        "data/13113_shibuya-ku_pref_2023_citygml_2_op/udx/bldg/53393596_bldg_6697_op.gml",
    )
    .unwrap();

    let mut set = SetOnMemory::new();

    println!("パース開始");

    let buildings: Vec<_> = BuildingParser::new(target).collect();

    println!("パース終了");

    let all_ids: HashSet<_> = buildings
        .into_par_iter()
        .filter_map(|ele| {
            let solid = Solid::new(ele.1.surfaces, 0.0).ok()?;
            let ids: Vec<_> = solid.range_ids(25).ok()?.collect();
            Some(ids)
        })
        .flatten()
        .collect();

    println!("挿入中");

    // --- 挿入と計測のセクション ---
    println!("挿入中（計測開始）");

    let mut log_file = File::create("insertion_times.csv").unwrap();
    let mut log_writer = BufWriter::new(log_file);

    // ヘッダーの書き出し
    writeln!(log_writer, "index,duration_ns").unwrap();

    let mut count = 0;

    let all = all_ids.iter().count();

    for id in all_ids {
        let start = Instant::now();
        set.insert(&id);
        let duration = start.elapsed().as_nanos(); // ナノ秒単位で計測

        // インデックスと時間を書き出し
        writeln!(log_writer, "{},{}", count, duration).unwrap();
        count += 1;
        if count % 100 == 0 {
            println!("{}/{}", count, all)
        }
    }
    log_writer.flush().unwrap();

    println!("書き出し中");

    let file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(file);

    for range_id in set.optimize_range_ids() {
        write!(writer, "{},", range_id).unwrap();
    }
}
