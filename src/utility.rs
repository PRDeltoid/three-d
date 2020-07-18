use crate::math::*;

pub struct BoundingBox {
    pub min_values: Vec2i,
    pub max_values: Vec2i
}

/// Find the bounding box of a given triangle formed by 2 screen coordinates (x,y)
pub fn find_bounding_box(triangle: &Triangle2d) -> BoundingBox
{
    BoundingBox {
        min_values: find_min(&triangle),
        max_values: find_max(&triangle)
    }
}

/// Converts (x,y,z) world coordinates to screen (pixel) coordinates (x,y)
pub fn find_screen_coordinates(vertex: &Vec3f, width: u32, height: u32) -> Vec2i {
    //println!("Finding screen coordinates for {} {}", vertex.x, vertex.y);

    let x = ((vertex.x + 1.0) * width as f32 / 2.0) as i32;
    let y = ((vertex.y + 1.0) * height as f32 / 2.0) as i32;

    Vec2i {
        x,
        y
    }
}

pub fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
