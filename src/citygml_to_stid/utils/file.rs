use crate::citygml_to_stid::bldg::BldgStorage;
use crate::citygml_to_stid::models::bldg::BuildingInfo;

use serde_json::{Value, json};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::path::Path;

pub fn save_building_info_json(
    count: i32,
    building_info: &BuildingInfo,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    // println!("export_name : {}", export_name);
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    // println!("safe_name : {}", safe_name);
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    //IDデータのみを拾う

    let mut file = File::create("voxels.txt")?;

    // 各 voxel を改行付きで書き出す
    for voxel in &building_info.stid_set {
        writeln!(file, "{},", voxel)?;
    }

    let file_path = dir_path.join(format!("{}.json", safe_name));

    let mut existing: Value = if let Ok(mut f) = File::open(&file_path) {
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        if buf.trim().is_empty() {
            json!({})
        } else {
            serde_json::from_str(&buf)?
        }
    } else {
        json!({})
    };

    existing[&count.to_string()] = json!({
        "id": building_info.building_id,
        "stid_set": building_info.stid_set.iter().map(|stid| stid.to_string()).collect::<Vec<String>>(),
        "attributes": building_info.attribute_info_map
    });

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    f.write_all(existing.to_string().as_bytes())?;
    Ok(())
}

pub fn save_building_infos_json(
    building_infos: Vec<BldgStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    // println!("export_name : {}", export_name);
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    // println!("safe_name : {}", safe_name);
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;
    
    let file_path = dir_path.join(format!("{}.json", safe_name));
    
    let mut data = Value::Array([].to_vec());

    for building_info in building_infos {
    data[building_info.count.to_string()] = json!({
        "id": building_info.building_info.building_id,
        "stid_set": building_info.building_info.stid_set.iter().map(|stid| stid.to_string()).collect::<Vec<String>>(),
        "attributes": building_info.building_info.attribute_info_map
    });
}

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    f.write_all(data.to_string().as_bytes())?;

    Ok(())
}
