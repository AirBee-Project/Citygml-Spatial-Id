use kasane_logic::id::{SpaceTimeId, coordinates::Point};
use kasane_logic::function::triangle::triangle;
use std::collections::HashSet;

pub fn citygml_polygon_to_ids(z: u8, vertices: &[Point]) -> HashSet<SpaceTimeId> {
    let mut all_ids = HashSet::new();
    if vertices.len() < 3 || vertices.iter().any(|p| p.altitude == 0.) {
        return all_ids;
    }

    let a = z_minus_point(&vertices[0]);
    for i in 1..vertices.len() - 1 {
        let b = z_minus_point(&vertices[i]);
        let c = z_minus_point(&vertices[i + 1]);
        all_ids.extend(triangle(z, a, b, c));
    }
    all_ids
}

fn z_minus_point(point: &Point) -> Point {
    Point {
        latitude: point.latitude,
        longitude: point.longitude,
        altitude: point.altitude - 77.0,
    }
}
