use crate::citygml_to_stid::models::bldg::BuildingInfo;
use crate::citygml_to_stid::models::types::CodeSpaceCache;
use crate::citygml_to_stid::utils::{cache, file, geometory, xml_parser};
use crate::citygml_to_stid::utils::find_code_space_tags::CodeSpaceContext;
use std::collections::{HashMap, HashSet};
// use std::time::Instant;

use quick_xml::{events::Event, reader::Reader};
use rayon::prelude::*;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct BldgStorage {
    pub count: i32,
    pub building_info: BuildingInfo,
}

pub fn bldg_info(num_of_parallel: usize) -> Result<Option<BuildingInfo>, Box<dyn Error>> {
    let base_dir = Path::new("CityData")
        // .join("citygml-to-stid/CityData/13109_shinagawa-ku_city_2024_citygml_1_op")
        .join("13109_shinagawa-ku_city_2024_citygml_1_op")
        .join("udx")
        .join("bldg");

    let files: Vec<PathBuf> = fs::read_dir(&base_dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension().is_some_and(|ext| ext == "gml")).then_some(path)
        })
        .take(num_of_parallel) 
        .collect();

    files.par_iter().for_each(|file_path| {
        if file_path.extension().is_some_and(|ext| ext == "gml") {
            if let Err(e) = process_one_file(file_path) {
                eprintln!("Error processing {}: {:?}", file_path.display(), e);
            }
        }
    });

    // Ok(Some(buildinginfo))
    //複数回回すとなると、何返せばいいかわかんないので、いったん適当
    Ok(Some(BuildingInfo {
        building_id: "end".to_string(),
        stid_set: HashSet::new(),
        attribute_info_map: HashMap::new(),
    }))
}

fn process_one_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.config_mut().trim_text(true);

    let mut storage: Vec<BldgStorage> = Vec::new();
    let mut buf = Vec::<u8>::new();
    let mut in_building = false;

    let mut building_count = 0;
    let mut buildinginfo = BuildingInfo {
        building_id: String::new(),
        stid_set: HashSet::new(),
        attribute_info_map: HashMap::new(),
    };

    let mut current_tag: Option<Vec<u8>> = None;
    // let mut uro_total = std::time::Duration::ZERO;
    // let mut poslist_total = std::time::Duration::ZERO;

    let mut code_space_cache: CodeSpaceCache = HashMap::new();
    let mut codespace_ctx = CodeSpaceContext::new();

    loop {
        let ev = reader.read_event_into(&mut buf)?;
        match ev {
            Event::Start(e) => {
                let tag_name: Vec<u8> = e.name().as_ref().to_vec();
                current_tag = Some(tag_name.clone());
                let attrs: Vec<_> = e.attributes().filter_map(|a| a.ok()).collect();

                if tag_name.as_slice() == b"bldg:Building" && !in_building {
                    in_building = true;
                    for a in &attrs {
                        if a.key.as_ref() == b"gml:id" {
                            buildinginfo.building_id = a.unescape_value()?.to_string();
                        }
                    }
                }

                codespace_ctx.on_start(&tag_name, &attrs, file_path);
                // gml:posList, measuredHeight, yearOfConstruction は Text で処理
            }
            Event::Text(t) => {
                if in_building {
                    let text_val = t.decode().unwrap().into_owned();
                    // println!("{:?}",text_val);
                    if let Some(mapped) =
                        codespace_ctx.resolve_text(&text_val, &mut code_space_cache)?
                    {
                        if let Some(tag_bytes) = &current_tag {
                            if let Ok(tag_str) = std::str::from_utf8(tag_bytes) {
                                buildinginfo
                                    .attribute_info_map
                                    .insert(tag_str.to_string(), mapped);
                            }
                        }
                    } else if let Some(tag_name) = &current_tag {
                        if tag_name.as_slice() == b"gml:posList" {
                            // let start = Instant::now();
                            let points = xml_parser::parse_points(&text_val).unwrap();

                            // let start_geo = Instant::now();
                            let ids = geometory::citygml_polygon_to_ids(25, &points);
                            // println!("  -> geometry変換時間: {:.3?}", start_geo.elapsed());
                            buildinginfo.stid_set.extend(ids);
                            // poslist_total += start.elapsed();
                        }
                    }
                }
            }
            Event::End(e) => {
                let name = e.name();
                let tag_name = name.as_ref();

                if let Some(tag_name_bytes) = &current_tag {
                    if tag_name_bytes.as_slice() == tag_name {
                        current_tag = None;
                    }
                }

                codespace_ctx.on_end(tag_name);

                if in_building && tag_name == b"bldg:Building" {
                    println!("building count : {}", building_count);
                    storage.push(BldgStorage {
                        count: building_count,
                        building_info: buildinginfo.clone(),
                    });
                    building_count += 1;
                    in_building = false;

                    //澤村：stid_setが累積しないようにする
                    buildinginfo = BuildingInfo {
                        building_id: String::new(),
                        stid_set: HashSet::new(),
                        attribute_info_map: HashMap::new(),
                    };
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    
    file::save_building_infos_json(storage, format!("{}_stid", file_path.display()))?;
    // println!(
    //     "File: {}\n  uroタグ処理時間: {:.3?}\n  posListタグ処理時間: {:.3?}\n",
    //     file_path.display(),
    //     uro_total,
    //     poslist_total
    // );

    Ok(())
}


// use crate::citygml_to_stid::models::bldg::BuildingInfo;
// use crate::citygml_to_stid::models::types::CodeSpaceCache;
// use crate::citygml_to_stid::utils::{cache, file, geometory, xml_parser};
// use regex::bytes::Regex;
// use std::collections::{HashMap, HashSet};
// // use std::time::Instant;

// use quick_xml::{events::Event, reader::Reader};
// use rayon::prelude::*;
// use std::error::Error;
// use std::fs::{self, File};
// use std::io::BufReader;
// use std::path::{Path, PathBuf};

// pub struct BldgStorage {
//     pub count: i32,
//     pub building_info: BuildingInfo,
// }

// pub fn bldg_info(num_of_parallel: usize) -> Result<Option<BuildingInfo>, Box<dyn Error>> {
//     let base_dir = Path::new("CityData")
//         // .join("citygml-to-stid/CityData/13109_shinagawa-ku_city_2024_citygml_1_op")
//         .join("13109_shinagawa-ku_city_2024_citygml_1_op")
//         .join("udx")
//         .join("bldg");

//     let files: Vec<PathBuf> = fs::read_dir(&base_dir)?
//         .filter_map(|entry| {
//             let path = entry.ok()?.path();
//             (path.extension().is_some_and(|ext| ext == "gml")).then_some(path)
//         })
//         .take(num_of_parallel) 
//         .collect();

//     files.par_iter().for_each(|file_path| {
//         if file_path.extension().is_some_and(|ext| ext == "gml") {
//             if let Err(e) = process_one_file(file_path) {
//                 eprintln!("Error processing {}: {:?}", file_path.display(), e);
//             }
//         }
//     });

//     // Ok(Some(buildinginfo))
//     //複数回回すとなると、何返せばいいかわかんないので、いったん適当
//     Ok(Some(BuildingInfo {
//         building_id: "end".to_string(),
//         stid_set: HashSet::new(),
//         attribute_info_map: HashMap::new(),
//     }))
// }

// fn process_one_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut reader = Reader::from_reader(BufReader::new(file));
//     reader.config_mut().trim_text(true);

//     let mut storage: Vec<BldgStorage> = Vec::new();
//     let mut buf = Vec::<u8>::new();
//     let mut in_building = false;
//     let mut in_uro = false;
//     let mut current_code_space_path: Option<PathBuf> = None;

//     let mut building_count = 0;
//     let mut buildinginfo = BuildingInfo {
//         building_id: String::new(),
//         stid_set: HashSet::new(),
//         attribute_info_map: HashMap::new(),
//     };

//     let re_uro = Regex::new(r"^uro:.*$").unwrap();
//     let mut current_tag: Option<Vec<u8>> = None;
//     // let mut uro_total = std::time::Duration::ZERO;
//     // let mut poslist_total = std::time::Duration::ZERO;

//     let mut code_space_cache: CodeSpaceCache = HashMap::new();

//     loop {
//         let ev = reader.read_event_into(&mut buf)?;
//         match ev {
//             Event::Start(e) => {
//                 let tag_name: Vec<u8> = e.name().as_ref().to_vec();
//                 current_tag = Some(tag_name.clone());
//                 let attrs: Vec<_> = e.attributes().filter_map(|a| a.ok()).collect();

//                 if tag_name.as_slice() == b"bldg:Building" && !in_building {
//                     in_building = true;
//                     for a in &attrs {
//                         if a.key.as_ref() == b"gml:id" {
//                             buildinginfo.building_id = a.unescape_value()?.to_string();
//                         }
//                     }
//                 }

//                 if re_uro.is_match(&tag_name) {
//                     in_uro = true;
//                     current_code_space_path = attrs.iter().find_map(|a| {
//                         if a.key.as_ref() == b"codeSpace" {
//                             Some(
//                                 file_path
//                                     .parent()?
//                                     .join(a.unescape_value().ok()?.as_ref())
//                                     .canonicalize()
//                                     .ok()?,
//                             )
//                         } else {
//                             None
//                         }
//                     });
//                 }
//                 // gml:posList, measuredHeight, yearOfConstruction は Text で処理
//             }
//             Event::Text(t) => {
//                 if in_building {
//                     let text_val = t.decode().unwrap().into_owned();
//                     // println!("{:?}",text_val);
//                     if in_uro {
//                         // let start = Instant::now();
//                         if let Some(abs_path) = &current_code_space_path {
//                             let code_map = cache::get_code_map(&mut code_space_cache, abs_path)?;
//                             let name = code_map.get(&text_val).unwrap_or(&text_val);
//                             if let Some(tag_bytes) = &current_tag {
//                                 if let Ok(tag_str) = std::str::from_utf8(tag_bytes) {
//                                     buildinginfo
//                                         .attribute_info_map
//                                         .insert(tag_str.to_string(), name.clone());
//                                 }
//                             }
//                         }
//                         // uro_total += start.elapsed();
//                     } else if let Some(tag_name) = &current_tag {
//                         if tag_name.as_slice() == b"gml:posList" {
//                             // let start = Instant::now();
//                             let points = xml_parser::parse_points(&text_val).unwrap();

//                             // let start_geo = Instant::now();
//                             let ids = geometory::citygml_polygon_to_ids(25, &points);
//                             // println!("  -> geometry変換時間: {:.3?}", start_geo.elapsed());
//                             buildinginfo.stid_set.extend(ids);
//                             // poslist_total += start.elapsed();
//                         }
//                     }
//                 }
//             }
//             Event::End(e) => {
//                 let name = e.name();
//                 let tag_name = name.as_ref();

//                 if let Some(tag_name_bytes) = &current_tag {
//                     if tag_name_bytes.as_slice() == tag_name {
//                         current_tag = None;
//                     }
//                 }

//                 if in_uro && re_uro.is_match(tag_name) {
//                     in_uro = false;
//                     current_code_space_path = None;
//                 }

//                 if in_building && tag_name == b"bldg:Building" {
//                     println!("building count : {}", building_count);
//                     storage.push(BldgStorage {
//                         count: building_count,
//                         building_info: buildinginfo.clone(),
//                     });
//                     building_count += 1;
//                     in_building = false;

//                     //澤村：stid_setが累積しないようにする
//                     buildinginfo = BuildingInfo {
//                         building_id: String::new(),
//                         stid_set: HashSet::new(),
//                         attribute_info_map: HashMap::new(),
//                     };
//                 }
//             }
//             Event::Eof => break,
//             _ => {}
//         }
//         buf.clear();
//     }
    
//     file::save_building_infos_json(storage, format!("{}_stid", file_path.display()))?;
//     // println!(
//     //     "File: {}\n  uroタグ処理時間: {:.3?}\n  posListタグ処理時間: {:.3?}\n",
//     //     file_path.display(),
//     //     uro_total,
//     //     poslist_total
//     // );

//     Ok(())
// }
