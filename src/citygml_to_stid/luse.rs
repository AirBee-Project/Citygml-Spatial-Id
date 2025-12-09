use crate::citygml_to_stid::models::luse::LuseInfo;
use crate::citygml_to_stid::models::types::CodeSpaceCache;
use crate::citygml_to_stid::utils::{cache, file, geometory, xml_parser};
use crate::citygml_to_stid::utils::find_code_space_tags::CodeSpaceContext;

use quick_xml::{events::Event, reader::Reader};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct LuseStorage {
    pub count: i32,
    pub luse_info: LuseInfo,
}

pub fn luse_info(num_of_parallel: usize) -> Result<Option<LuseInfo>, Box<dyn Error>> {
    let base_dir = Path::new("CityData")
        .join("13109_shinagawa-ku_city_2024_citygml_1_op")
        .join("udx")
        .join("luse");

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

    Ok(None)
}

fn process_one_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.config_mut().trim_text(true);

    let mut storage: Vec<LuseStorage> = Vec::new();
    let mut buf = Vec::<u8>::new();
    let mut in_landuse = false;
    let mut current_tag: Option<Vec<u8>> = None;

    let mut landuse_count = 0;
    let mut luseinfo = LuseInfo {
        landuse_id: String::new(),
        stid_set: HashSet::new(),
        attribute_info_map: HashMap::new(),
    };

    let mut code_space_cache: CodeSpaceCache = HashMap::new();
    let mut codespace_ctx = CodeSpaceContext::new();

    loop {
        let ev = reader.read_event_into(&mut buf)?;
        match ev {
            Event::Start(e) => {
                let tag_name: Vec<u8> = e.name().as_ref().to_vec();
                current_tag = Some(tag_name.clone());
                let attrs: Vec<_> = e.attributes().filter_map(|a| a.ok()).collect();

                if tag_name.as_slice() == b"luse:LandUse" && !in_landuse {
                    in_landuse = true;
                    for a in &attrs {
                        if a.key.as_ref() == b"gml:id" {
                            luseinfo.landuse_id = a.unescape_value()?.to_string();
                        }
                    }
                }

                codespace_ctx.on_start(&tag_name, &attrs, file_path);
            }
            Event::Text(t) => {
                if in_landuse {
                    let text_val = t.decode().unwrap().into_owned();

                    if let Some(mapped) =
                        codespace_ctx.resolve_text(&text_val, &mut code_space_cache)?
                    {
                        if let Some(tag_bytes) = &current_tag {
                            if let Ok(tag_str) = std::str::from_utf8(tag_bytes) {
                                luseinfo
                                    .attribute_info_map
                                    .insert(tag_str.to_string(), mapped);
                            }
                        }
                    } else if let Some(tag_name) = &current_tag {
                        if tag_name.as_slice() == b"gml:posList" {
                            let points = xml_parser::parse_points(&text_val).unwrap();
                            let ids = geometory::citygml_polygon_to_ids(25, &points);
                            luseinfo.stid_set.extend(ids);
                        } else if tag_name.starts_with(b"urf:")
                            || tag_name.starts_with(b"uro:")
                            || tag_name.starts_with(b"luse:")
                        {
                            if let Ok(tag_str) = std::str::from_utf8(tag_name) {
                                luseinfo
                                    .attribute_info_map
                                    .insert(tag_str.to_string(), text_val);
                            }
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

                if in_landuse && tag_name == b"luse:LandUse" {
                    println!("luse landuse count : {}", landuse_count);
                    storage.push(LuseStorage {
                        count: landuse_count,
                        luse_info: luseinfo.clone(),
                    });
                    landuse_count += 1;
                    in_landuse = false;

                    luseinfo = LuseInfo {
                        landuse_id: String::new(),
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

    file::save_luse_infos_json(storage, format!("{}_stid", file_path.display()))?;
    Ok(())
}
