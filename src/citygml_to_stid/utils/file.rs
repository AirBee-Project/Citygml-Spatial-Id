use crate::citygml_to_stid::bldg::BldgStorage;
use crate::citygml_to_stid::models::bldg::BuildingInfo;

use crate::citygml_to_stid::brid::BridStorage;
use crate::citygml_to_stid::models::brid::BridgeInfo;

use crate::citygml_to_stid::dem::DemStorage;
use crate::citygml_to_stid::models::dem::DemInfo;

use crate::citygml_to_stid::fld::FldStorage;
use crate::citygml_to_stid::models::fld::FldInfo;

use crate::citygml_to_stid::frn::FrnStorage;
use crate::citygml_to_stid::models::frn::FrnInfo;

use crate::citygml_to_stid::htd::HtdStorage;
use crate::citygml_to_stid::models::htd::HtdInfo;

use crate::citygml_to_stid::lsld::LsldStorage;
use crate::citygml_to_stid::models::lsld::LsldInfo;

use crate::citygml_to_stid::luse::LuseStorage;
use crate::citygml_to_stid::models::luse::LuseInfo;

use crate::citygml_to_stid::tran::TranStorage;
use crate::citygml_to_stid::models::tran::TranInfo;

use crate::citygml_to_stid::urf::UrfStorage;
use crate::citygml_to_stid::models::urf::UrfInfo;


use serde_json::{Value, json};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::path::Path;

//まとめて保存する用
pub fn save_building_infos_json(
    building_infos: Vec<BldgStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    // ファイルが開けないほど重くなってしまうため、チャンクごとに分割
    for (i, chunk) in building_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for building_info in chunk {
            data[building_info.count.to_string()] = json!({
                "id": building_info.building_info.building_id,
                "stid_set": building_info.building_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": building_info.building_info.attribute_info_map
            });
        }

        // ファイル名にチャンク番号を付与
        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_bridge_infos_json(
    bridge_infos: Vec<BridStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in bridge_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for bridge in chunk {
            data[bridge.count.to_string()] = json!({
                "id": bridge.bridge_info.bridge_id,
                "stid_set": bridge.bridge_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": bridge.bridge_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_dem_infos_json(
    dem_infos: Vec<DemStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in dem_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for dem in chunk {
            data[dem.count.to_string()] = json!({
                "id": dem.dem_info.relief_id,
                "stid_set": dem.dem_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": dem.dem_info.attribute_info_map,
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_fld_infos_json(
    fld_infos: Vec<FldStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in fld_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for fld in chunk {
            data[fld.count.to_string()] = json!({
                "id": fld.fld_info.waterbody_id,
                "stid_set": fld.fld_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": fld.fld_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_frn_infos_json(
    frn_infos: Vec<FrnStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in frn_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for frn in chunk {
            data[frn.count.to_string()] = json!({
                "id": frn.frn_info.furniture_id,
                "stid_set": frn.frn_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": frn.frn_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_htd_infos_json(
    htd_infos: Vec<HtdStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in htd_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for htd in chunk {
            data[htd.count.to_string()] = json!({
                "id": htd.htd_info.waterbody_id,
                "stid_set": htd.htd_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": htd.htd_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_lsld_infos_json(
    lsld_infos: Vec<LsldStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in lsld_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for lsld in chunk {
            data[lsld.count.to_string()] = json!({
                "id": lsld.lsld_info.area_id,
                "stid_set": lsld.lsld_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": lsld.lsld_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_luse_infos_json(
    luse_infos: Vec<LuseStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in luse_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for luse in chunk {
            data[luse.count.to_string()] = json!({
                "id": luse.luse_info.landuse_id,
                "stid_set": luse.luse_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": luse.luse_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_tran_infos_json(
    tran_infos: Vec<TranStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in tran_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for tran in chunk {
            data[tran.count.to_string()] = json!({
                "id": tran.tran_info.road_id,
                "stid_set": tran.tran_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": tran.tran_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

pub fn save_urf_infos_json(
    urf_infos: Vec<UrfStorage>,
    export_name: String,
) -> Result<(), Box<dyn Error>> {
    let safe_name = export_name.replace('\\', "_").replace('/', "_");
    let dir_path = Path::new("stid_json");
    create_dir_all(dir_path)?;

    let chunk_size = 50;
    for (i, chunk) in urf_infos.chunks(chunk_size).enumerate() {
        let mut data = Value::Object(serde_json::Map::new());

        for urf in chunk {
            data[urf.count.to_string()] = json!({
                "id": urf.urf_info.district_id,
                "stid_set": urf.urf_info
                    .stid_set
                    .iter()
                    .map(|stid| stid.to_string())
                    .collect::<Vec<String>>(),
                "attributes": urf.urf_info.attribute_info_map
            });
        }

        let file_path = dir_path.join(format!("{}_part{}.json", safe_name, i + 1));

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        f.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    }

    Ok(())
}

//単体保存用
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