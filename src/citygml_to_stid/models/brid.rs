use std::collections::{HashMap, HashSet};
use kasane_logic::space_time_id::SpaceTimeId;

#[derive(Clone)]
pub struct BridgeInfo {
    pub bridge_id: String,
    pub stid_set: HashSet<SpaceTimeId>,
    pub attribute_info_map: HashMap<String, String>,
}
