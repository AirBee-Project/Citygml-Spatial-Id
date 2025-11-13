use kasane_logic::function::triangle::triangle;
use kasane_logic::point::{Coordinate, Point};
use kasane_logic::space_time_id::SpaceTimeId;
use std::collections::HashSet;
// use std::time::Instant;

pub fn citygml_polygon_to_ids(z: u8, vertices: &[Coordinate]) -> HashSet<SpaceTimeId> {
    let mut all_ids = HashSet::new();
    if vertices.len() < 3 || vertices.iter().any(|p| p.altitude == 0.) {
        return all_ids;
    }
    // let start = Instant::now();
    let a = z_minus_point(&vertices[0]);
    for i in 1..vertices.len() - 1 {
        let b = z_minus_point(&vertices[i]);
        let c = z_minus_point(&vertices[i + 1]);
        // println!("a:{:?}, b:{:?}, c:{:?}",a,b,c);
        // let t_start = Instant::now();
        all_ids.extend(triangle(
            z,
            Point::Coordinate(a),
            Point::Coordinate(b),
            Point::Coordinate(c),
        ));
        // let t_elapsed = t_start.elapsed();
        // if t_elapsed.as_secs_f32() > 0.01 {
        //     println!("  triangle() took {:.3?} (i={})", t_elapsed, i);
        // }
    }
    // println!(
    //     "→ citygml_polygon_to_ids: {} vertices, total {:.3?}",
    //     vertices.len(),
    //     start.elapsed()
    // );
    all_ids
}

fn z_minus_point(point: &Coordinate) -> Coordinate {
    Coordinate {
        latitude: point.latitude,
        longitude: point.longitude,
        altitude: point.altitude,
    }
}
