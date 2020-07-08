use crate::math::*;

pub struct BoundingBox {
    pub min_values: Vec2i,
    pub max_values: Vec2i
}

/// Find the bounding box of a given triangle formed by 2 screen coordinates (x,y)
pub fn find_bounding_box(vertexes: Vec<Vec2i>) -> BoundingBox
{
    BoundingBox {
        min_values: find_min(&vertexes),
        max_values: find_max(&vertexes)
    }
}

pub fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
