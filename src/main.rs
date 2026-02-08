use std::fs::File;

use citygml_spatial_id::theme::building::Building;

fn main() {
    let file = File::open(
        "data/11234_yashio-shi_pref_2023_citygml_2_op/udx/bldg/53396627_bldg_6697_op.gml",
    )
    .unwrap();

    let data = BuildingPa

    let result: Vec<_> = data.collect();

    println!("{:?}", result)
}
