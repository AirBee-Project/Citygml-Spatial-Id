use kasane_logic::id::{SpaceTimeId};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct BuildingInfo {
    pub building_id: String,
    pub stid_set: HashSet<SpaceTimeId>,
    pub attribute_info_map: HashMap<String, String>,
}

// impl BuildingInfo {
//     pub fn new() -> Self {
//         Self {
//             building_id: String::new(),
//             stid_set: HashSet::new(),
//             attribute_info_map: HashMap::new(),
//         }
//     }
// }
