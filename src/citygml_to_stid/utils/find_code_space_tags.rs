use crate::citygml_to_stid::models::types::CodeSpaceCache;
use crate::citygml_to_stid::utils::cache;
use quick_xml::events::attributes::Attribute;
use std::error::Error;
use std::path::{Path, PathBuf};

pub struct CodeSpaceContext {
    active: bool, //inside element with 'codeSpace'
    owner_tag: Option<Vec<u8>>,
    path: Option<PathBuf>, //codelist path
}

impl CodeSpaceContext {
    pub fn new() -> Self {
        Self {
            active: false,
            owner_tag: None,
            path: None,
        }
    }

    pub fn on_start(&mut self, tag_name: &[u8], attrs: &[Attribute], file_path: &Path) {
        let resolved_path = attrs.iter().find_map(|a| {
            if a.key.as_ref() == b"codeSpace" {
                let value = a.unescape_value().ok()?;
                let parent = file_path.parent()?;
                parent
                    .join(value.as_ref()) //Cow<str> -> &str
                    .canonicalize()
                    .ok()
            } else {
                None
            }
        });

        if let Some(path) = resolved_path {
            self.active = true;
            self.owner_tag = Some(tag_name.to_vec());
            self.path = Some(path);
        }
    }

    pub fn on_end(&mut self, tag_name: &[u8]) {
        if !self.active {
            return;
        }
        if let Some(owner) = &self.owner_tag {
            if owner.as_slice() == tag_name {
                self.active = false;
                self.owner_tag = None;
                self.path = None;
            }
        }
    }

    pub fn resolve_text(
        &self,
        raw_code: &str,
        cache_map: &mut CodeSpaceCache,
    ) -> Result<Option<String>, Box<dyn Error>> {
        if !self.active {
            return Ok(None);
        }
        let abs_path = match &self.path {
            Some(p) => p,
            None => return Ok(None),
        };

        let code_map = cache::get_code_map(cache_map, abs_path)?;

        let mapped = code_map
            .get(raw_code)
            .cloned()
            .unwrap_or_else(|| raw_code.to_string());

        Ok(Some(mapped))
    }
}