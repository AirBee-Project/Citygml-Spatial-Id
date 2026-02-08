use std::fs::File;

use citygml_spatial_id::theme::building::BuildingParser;

fn main() {
    let file = File::open(
        "data/13220_higashiyamato-shi_pref_2023_citygml_2_op/udx/bldg/53394382_bldg_6697_op.gml",
    )
    .unwrap();

    let data = BuildingParser::new(file);

    let result: Vec<_> = data.collect();

    println!("{:?}", result)
}
