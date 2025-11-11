use crate::citygml_to_stid::models::types::CodeSpaceCache;
use crate::citygml_to_stid::utils::code_space_parser;
use std::{collections::HashMap, error::Error, path::PathBuf};

pub fn get_code_map<'a>(
    cache: &'a mut CodeSpaceCache,
    path: &'a PathBuf,
) -> Result<&'a HashMap<String, String>, Box<dyn Error>> {
    if !cache.contains_key(path) {
        let map = code_space_parser::parse_code_space(path.clone())?;
        cache.insert(path.clone(), map);
    }
    Ok(cache.get(path).unwrap())
}
