use std::collections::HashSet;

use earcutr::earcut;
use kasane_logic::{Coordinate, SingleId, triangle};

pub fn polygon(z: u8, surfaces: &[Vec<Coordinate>]) -> HashSet<SingleId> {
    let mut result = HashSet::new();

    for surface in surfaces {
        let ids = surface_to_ids(z, surface);
        result.extend(ids);
    }

    result
}

/// 単一 Polygon を空間ID集合に変換
fn surface_to_ids(z: u8, vertices: &[Coordinate]) -> HashSet<SingleId> {
    if vertices.len() < 3 {
        return HashSet::new();
    }
    let normal = calculate_normal_newell(vertices);
    let flat_coords = project_vertices_to_2d(vertices, normal);
    let indices = match earcut(&flat_coords, &[], 2) {
        Ok(i) => i,
        Err(_) => return HashSet::new(),
    };

    let mut ids = HashSet::new();
    for tri in indices.chunks_exact(3) {
        let a = vertices[tri[0]];
        let b = vertices[tri[1]];
        let c = vertices[tri[2]];

        let iter = triangle::triangle(z, a, b, c).unwrap();
        ids.extend(iter);
    }

    ids
}

/// 3D Polygon を最も面積が保たれる 2D 平面に投影
fn project_vertices_to_2d(vertices: &[Coordinate], normal: (f64, f64, f64)) -> Vec<f64> {
    let mut flat = Vec::with_capacity(vertices.len() * 2);
    let drop_axis = dominant_axis(normal);
    for v in vertices {
        match drop_axis {
            Axis::X => {
                flat.push(v.as_latitude());
                flat.push(v.as_altitude());
            }
            Axis::Y => {
                flat.push(v.as_longitude());
                flat.push(v.as_altitude());
            }
            Axis::Z => {
                flat.push(v.as_longitude());
                flat.push(v.as_latitude());
            }
        }
    }

    flat
}

enum Axis {
    X,
    Y,
    Z,
}

fn dominant_axis(n: (f64, f64, f64)) -> Axis {
    let (x, y, z) = (n.0.abs(), n.1.abs(), n.2.abs());

    if x >= y && x >= z {
        Axis::X
    } else if y >= z {
        Axis::Y
    } else {
        Axis::Z
    }
}

fn calculate_normal_newell(vertices: &[Coordinate]) -> (f64, f64, f64) {
    let mut nx = 0.0;
    let mut ny = 0.0;
    let mut nz = 0.0;

    for i in 0..vertices.len() {
        let cur = &vertices[i];
        let next = &vertices[(i + 1) % vertices.len()];

        nx += (cur.as_latitude() - next.as_latitude()) * (cur.as_altitude() + next.as_altitude());
        ny += (cur.as_altitude() - next.as_altitude()) * (cur.as_longitude() + next.as_longitude());
        nz += (cur.as_longitude() - next.as_longitude()) * (cur.as_latitude() + next.as_latitude());
    }

    (nx, ny, nz)
}
