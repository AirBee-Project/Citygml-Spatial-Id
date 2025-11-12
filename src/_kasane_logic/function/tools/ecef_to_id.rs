use crate::{
    function::tools::{ECEF, ecef_to_point::ecef_to_point, point_to_id::point_to_id},
    id::SpaceTimeId,
};

pub fn ecef_to_id(z: u8, ecef: ECEF) -> SpaceTimeId {
    point_to_id(z, ecef_to_point(ecef))
}
