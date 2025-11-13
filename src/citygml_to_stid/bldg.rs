use crate::citygml_to_stid::models::types::CodeSpaceCache;
use crate::citygml_to_stid::utils::{cache, code_space_parser, file, geometory, xml_parser};
use crate::citygml_to_stid::models::bldg::BuildingInfo;
use regex::bytes::Regex;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

use quick_xml::{events::Event, reader::Reader};
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct BldgStorage {
    pub count: i32,
    pub building_info: BuildingInfo,
}

pub fn bldg_info() -> Result<Option<BuildingInfo>, Box<dyn Error>> {
    let base_dir = Path::new("CityData")
        .join("10201_maebashi-shi_city_2023_citygml_2_op")
        // .join("13109_shinagawa-ku_city_2024_citygml_1_op")
        .join("udx")
        .join("bldg");

    // let mut file_count = 0;
    // for entry in fs::read_dir(&base_dir)? {

    let mut count = 0;
    let mut code_space_cache: CodeSpaceCache = HashMap::new();


    let files: Vec<PathBuf> = fs::read_dir(&base_dir)?
        .filter_map(|entry| {
        let path = entry.ok()?.path();
        (path.extension().is_some_and(|ext| ext == "gml")).then_some(path)
    })
    .take(1) // 最初の1個だけ処理
    .collect();

    // files.par_iter().for_each(|file_path| {
    for file_path in files {
        let mut storage:Vec<BldgStorage> = Vec::new();
        // let entry = entry_result;
        // let file_path = entry.path();
        // println!("{:?}", file_path);
        if file_path.extension().is_some_and(|ext| ext == "gml") {
            // file_count += 1;
            let file = File::open(&file_path);
            let mut reader = Reader::from_reader(BufReader::new(file.unwrap()));
            reader.config_mut().trim_text(true);

            let mut buf = Vec::<u8>::new();
            let mut in_building = false;
            let mut in_uro = false;
            let mut current_code_space_path: Option<PathBuf> = None;

            let mut building_count = 0;
            let mut buildinginfo = BuildingInfo {
                building_id: String::new(),
                stid_set: HashSet::new(),
                attribute_info_map: HashMap::new(),
            };

            let re_uro = Regex::new(r"^uro:.*$").unwrap();
            let mut current_tag: Option<Vec<u8>> = None;

            loop {
                let ev = reader.read_event_into(&mut buf).unwrap();
                match ev {
                    Event::Start(e) => {
                        let tag_name: Vec<u8> = e.name().as_ref().to_vec();
                        current_tag = Some(tag_name.clone());
                        let attrs: Vec<_> = e.attributes().filter_map(|a| a.ok()).collect();

                        // bldg:Building タグ開始
                        if tag_name.as_slice() == b"bldg:Building" && !in_building {
                            in_building = true;
                            for a in &attrs {
                                if a.key.as_ref() == b"gml:id" {
                                    buildinginfo.building_id = a.unescape_value().unwrap().to_string();
                                }
                            }
                        }

                        // uro:BuildingDetailAttributeタグ 開始（このタグの中身は属性情報）
                        if re_uro.is_match(&tag_name) {
                            in_uro = true;
                            current_code_space_path = attrs.iter().find_map(|a| {
                                if a.key.as_ref() == b"codeSpace" {
                                    Some(
                                        file_path
                                            .parent()
                                            .unwrap_or_else(|| Path::new("."))
                                            .join(a.unescape_value().ok()?.as_ref())
                                            .canonicalize()
                                            .ok()?,
                                    )
                                } else {
                                    None
                                }
                            });
                        }

                        // gml:posList, measuredHeight, yearOfConstruction は Text で処理
                    }
                    Event::Text(t) => {
                        if in_building {
                            let text_val = t.decode().unwrap().into_owned();
                            // println!("{:?}",text_val);
                            if in_uro {
                                if let Some(abs_path) = &current_code_space_path {
                                    let code_map = cache::get_code_map(&mut code_space_cache, abs_path)?;
                                    let name = code_map.get(&text_val).unwrap_or(&text_val);
                                    if let Some(tag_bytes) = &current_tag {
                                        if let Ok(tag_str) = std::str::from_utf8(tag_bytes) {
                                            buildinginfo
                                                .attribute_info_map
                                                .insert(tag_str.to_string(), name.clone());
                                        }
                                    }
                                    // let code_map = code_space_parser::parse_code_space(abs_path.clone()).unwrap_or_default();
                                    // let name = code_map.get(&text_val).unwrap_or(&text_val);
                                    // if let Some(tag_bytes) = &current_tag {
                                    //     if let Ok(tag_str) = std::str::from_utf8(tag_bytes) {
                                    //         buildinginfo
                                    //             .attribute_info_map
                                    //             .insert(tag_str.to_string(), name.clone());
                                    //     }
                                    // }
                                }
                            } else if let Some(tag_name) = &current_tag {
                                if tag_name.as_slice() == b"gml:posList" {
                                    let points = xml_parser::parse_points(&text_val).unwrap();
                                    buildinginfo
                                        .stid_set
                                        .extend(geometory::citygml_polygon_to_ids(20, &points));
                                }
                            }
                        }
                    }
                    Event::End(e) => {
                        let name = e.name();
                        let tag_name = name.as_ref();

                        if let Some(tag_name) = &current_tag {
                            if tag_name.as_slice() == e.name().as_ref() {
                                current_tag = None;
                            }
                        }

                        if in_uro && re_uro.is_match(tag_name) {
                            in_uro = false;
                            current_code_space_path = None;
                        }

                        if in_building && tag_name == b"bldg:Building" {
                            // println!("file path :  {}", file_path.display());
                            // println!("building info : {:#?}",buildinginfo );
                            println!("building count : {}", building_count);
                            //一つ一つ保存
                            // file::save_building_info_json(building_count, &buildinginfo, format!("{}_stid", file_path.display()));
                            storage.push(BldgStorage {
                                count:building_count,
                                building_info: buildinginfo.clone()
                        });
                            building_count += 1;
                            in_building = false;
                            in_uro = false; //一周しかしないと意味がないが、複数回まわすことになった時に使う
                            
                            // break; // 最初の Building だけ処理
                        }
                    }
                    Event::Eof => break,
                    _ => {}
                }
                buf.clear();
            }
        }
        
        // if file_count == 1 {
        //     break;
        // }
        
        file::save_building_infos_json(storage, format!("{}_stid", file_path.display())).unwrap();
    }
        // });

    // }

    // let file_path: PathBuf = fs::read_dir(&base_dir)?
    //     .filter_map(|entry| {
    //         let entry = entry.ok()?;
    //         let path = entry.path();
    //         if path.extension().is_some_and(|ext| ext == "gml") {
    //             Some(path)
    //         } else {
    //             None
    //         }
    //     })
    //     .next()
    //     .ok_or_else(|| format!("No .gml files found in {:?}", base_dir))?;

    // Ok(Some(buildinginfo))
    //複数回回すとなると、何返せばいいかわかんないので、いったん適当
    Ok(Some(BuildingInfo {
        building_id: "end".to_string(),
        stid_set: HashSet::new(),
        attribute_info_map: HashMap::new(),
    }))
}


