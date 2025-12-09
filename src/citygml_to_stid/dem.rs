use crate::citygml_to_stid::models::dem::DemInfo;
use crate::citygml_to_stid::models::types::CodeSpaceCache;
use crate::citygml_to_stid::utils::{cache, file, geometory, xml_parser};
use crate::citygml_to_stid::utils::find_code_space_tags::CodeSpaceContext;
use std::collections::{HashMap, HashSet};

use quick_xml::{events::Event, reader::Reader};
use rayon::prelude::*;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct DemStorage {
    pub count: i32,
    pub dem_info: DemInfo,
}

pub fn dem_info(num_of_parallel: usize) -> Result<Option<DemInfo>, Box<dyn Error>> {
    let base_dir = Path::new("CityData")
        .join("13109_shinagawa-ku_city_2024_citygml_1_op")
        .join("udx")
        .join("dem");

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

    Ok(Some(DemInfo {
        relief_id: "end".to_string(),
        stid_set: HashSet::new(),
        attribute_info_map: HashMap::new(),
    }))
}

fn process_one_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.config_mut().trim_text(true);

    let mut storage: Vec<DemStorage> = Vec::new();
    let mut buf = Vec::<u8>::new();
    let mut in_relief = false;

    let mut dem_count = 0;
    let mut deminfo = DemInfo {
        relief_id: String::new(),
        stid_set: HashSet::new(),
        attribute_info_map: HashMap::new(),
    };

    let mut current_tag: Option<Vec<u8>> = None;

    let mut code_space_cache: CodeSpaceCache = HashMap::new();
    let mut codespace_ctx = CodeSpaceContext::new();

    loop {
        let ev = reader.read_event_into(&mut buf)?;
        match ev {
            Event::Start(e) => {
                let tag_name: Vec<u8> = e.name().as_ref().to_vec();
                current_tag = Some(tag_name.clone());
                let attrs: Vec<_> = e.attributes().filter_map(|a| a.ok()).collect();

                if tag_name.as_slice() == b"dem:ReliefFeature" && !in_relief {
                    in_relief = true;
                    for a in &attrs {
                        if a.key.as_ref() == b"gml:id" {
                            deminfo.relief_id = a.unescape_value()?.to_string();
                        }
                    }
                }

                codespace_ctx.on_start(&tag_name, &attrs, file_path);
            }
            Event::Text(t) => {
                if in_relief {
                    let text_val = t.decode().unwrap().into_owned();
                    if let Some(mapped) =
                        codespace_ctx.resolve_text(&text_val, &mut code_space_cache)?
                    {
                        if let Some(tag_bytes) = &current_tag {
                            if let Ok(tag_str) = std::str::from_utf8(tag_bytes) {
                                deminfo
                                    .attribute_info_map
                                    .insert(tag_str.to_string(), mapped);
                            }
                        }
                    } else if let Some(tag_name) = &current_tag {
                        if tag_name.as_slice() == b"gml:posList" {
                            let points = xml_parser::parse_points(&text_val).unwrap();
                            let ids = geometory::citygml_polygon_to_ids(25, &points);
                            deminfo.stid_set.extend(ids);
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

                if in_relief && tag_name == b"dem:ReliefFeature" {
                    println!("dem relief feature count : {}", dem_count);
                    storage.push(DemStorage {
                        count: dem_count,
                        dem_info: deminfo.clone(),
                    });
                    dem_count += 1;
                    in_relief = false;

                    deminfo = DemInfo {
                        relief_id: String::new(),
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

    file::save_dem_infos_json(storage, format!("{}_stid", file_path.display()))?;
    Ok(())
}
