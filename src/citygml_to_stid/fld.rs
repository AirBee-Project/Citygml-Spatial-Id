use crate::citygml_to_stid::models::fld::FldInfo;
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

pub struct FldStorage {
    pub count: i32,
    pub fld_info: FldInfo,
}

fn collect_gml_files(base: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(base)? {
        let path = entry?.path();
        if path.is_dir() {
            out.extend(collect_gml_files(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "gml") {
            out.push(path);
        }
    }
    Ok(out)
}

pub fn fld_info(num_of_parallel: usize) -> Result<Option<FldInfo>, Box<dyn Error>> {
    let base_dir = Path::new("CityData")
        .join("13109_shinagawa-ku_city_2024_citygml_1_op")
        .join("udx")
        .join("fld");

    let files = collect_gml_files(&base_dir)?
        .into_iter()
        .take(num_of_parallel)
        .collect::<Vec<_>>();

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

    let mut storage: Vec<FldStorage> = Vec::new();
    let mut buf = Vec::<u8>::new();
    let mut in_waterbody = false;
    let mut current_tag: Option<Vec<u8>> = None;

    let mut waterbody_count = 0;
    let mut fldinfo = FldInfo {
        waterbody_id: String::new(),
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

                if tag_name.as_slice() == b"wtr:WaterBody" && !in_waterbody {
                    in_waterbody = true;
                    for a in &attrs {
                        if a.key.as_ref() == b"gml:id" {
                            fldinfo.waterbody_id = a.unescape_value()?.to_string();
                        }
                    }
                }

                codespace_ctx.on_start(&tag_name, &attrs, file_path);
            }
            Event::Text(t) => {
                if in_waterbody {
                    let text_val = t.decode().unwrap().into_owned();

                    if let Some(mapped) =
                        codespace_ctx.resolve_text(&text_val, &mut code_space_cache)?
                    {
                        if let Some(tag_bytes) = &current_tag {
                            if let Ok(tag_str) = std::str::from_utf8(tag_bytes) {
                                fldinfo
                                    .attribute_info_map
                                    .insert(tag_str.to_string(), mapped);
                            }
                        }
                    } else if let Some(tag_name) = &current_tag {
                        if tag_name.as_slice() == b"gml:posList" {
                            let points = xml_parser::parse_points(&text_val).unwrap();
                            let ids = geometory::citygml_polygon_to_ids(25, &points);
                            fldinfo.stid_set.extend(ids);
                        } else if tag_name.starts_with(b"urf:") || tag_name.starts_with(b"uro:") {
                            if let Ok(tag_str) = std::str::from_utf8(tag_name) {
                                fldinfo
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

                if in_waterbody && tag_name == b"wtr:WaterBody" {
                    println!("fld waterbody count : {}", waterbody_count);
                    storage.push(FldStorage {
                        count: waterbody_count,
                        fld_info: fldinfo.clone(),
                    });
                    waterbody_count += 1;
                    in_waterbody = false;

                    fldinfo = FldInfo {
                        waterbody_id: String::new(),
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

    file::save_fld_infos_json(storage, format!("{}_stid", file_path.display()))?;
    Ok(())
}
