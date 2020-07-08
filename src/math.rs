use std::num;

pub struct Vec2i {
    pub x: i32,
    pub y: i32
}

pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Triangle {
    pub point_1: Vec2i,
    pub point_2: Vec2i,
    pub point_3: Vec2i
}

/// Find the minimum x and y values for a given triangle (lower bounding box)
pub fn find_min(values: &Vec<Vec2i>) -> Vec2i {
    let mut min_x_val = -1;
    let mut min_y_val = -1;
    for val in values {
        if val.x < min_x_val {
            min_x_val = val.x;
        }

        if val.y < min_y_val {
            min_y_val = val.y;
        }
    }

    Vec2i {
        x: min_x_val,
        y: min_y_val
    }
}

/// Find the maximum x and y values for a given triangle (upper bounding box)
pub fn find_max(values: &Vec<Vec2i>) -> Vec2i {
    let mut max_x_val = -1; // WIDTH-1;
    let mut max_y_val = -1; //HEIGHT-1;
    for val in values {
        if val.x > max_x_val {
            max_x_val = val.x;
        }

        if val.y > max_y_val {
            max_y_val = val.y;
        }
    }

    Vec2i {
        x: max_x_val,
        y: max_y_val
    }
}

/// Cross Product of two 3-component vectors
fn cross(vec1: Vec3f, vec2: Vec3f) -> Vec3f {
    Vec3f {
        x: vec1.y * vec2.z - vec1.z * vec2.y,
        y: vec1.z * vec2.x - vec1.x * vec2.z,
        z: vec1.x * vec2.y - vec1.y * vec2.x
    }
}

/// Computes the barycentric coordinates of a given set of vectors and a point
pub fn barycentric(triangle: Triangle, point: Vec2i) -> Vec3f
{
    let u: Vec3f = cross(Vec3f {
        x: (triangle.point_2.x - triangle.point_1.x) as f32,
        y: (triangle.point_2.x - triangle.point_1.x) as f32,
        z: (triangle.point_1.x - point.x) as f32
    },
     Vec3f {
         x: (triangle.point_3.y - triangle.point_1.y) as f32,
         y: (triangle.point_2.y - triangle.point_1.y) as f32,
         z: (triangle.point_1.y - point.y) as f32
     });

    /* `triangle` and `P` have integer value coordinates
       so `u.abs()` < 1 means `u.z` must be 0
       therefore, triangle is degenerate and we should not draw it */
    if (u.z.abs() as i32) < 1 {
        // Return early with a vector that has a -1 component (thus this point will not be drawn)
        return Vec3f {
            x: -1 as f32,
            y: 1 as f32,
            z: 1 as f32
        }
    }

    Vec3f {
        x: 1.0 - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z
    }
}
