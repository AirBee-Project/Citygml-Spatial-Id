use std::collections::{HashMap, HashSet};

use kasane_logic::space_time_id::SpaceTimeId;

#[derive(Clone)]

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
