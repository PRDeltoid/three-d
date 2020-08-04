use core::ops::Index;
use std::fmt;
use std::ops;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    data: Vec<f32>
}

impl Index<usize> for Matrix {
    type Output = [f32];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols .. (index+1) * self.cols]
    }

}

impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index * self.cols .. (index+1) * self.cols]
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut result: Matrix = Matrix::new(self.rows, rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                result[i][j] = 0.;
                for k in 0..self.cols {
                    result[i][j] += self[i][k]*rhs[k][j];
                }
            }
        }
        result
    }
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows: rows,
            cols: cols,
            data: Vec::new()
        }
    }

    pub fn to_vec(&self) -> Vec3f {
        Vec3f {
            x: self[0][0]/self[3][0],
            y: self[1][0]/self[3][0],
            z: self[2][0]/self[3][0]
        }

    }

    pub fn identity(size: usize) -> Matrix {
        let mut identity = Matrix { 
            rows: size,
            cols: size,
            data: Vec::new()
        };

        for i in 0..identity.rows {
            for j in 0..identity.cols {
                if i == j {
                    identity[i][j] = 1.;
                } else {
                    identity[i][j] = 0.;
                }
            }
        }

        identity
    }
}

#[derive(Clone, Copy)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

pub struct Vec2i {
    pub x: i32,
    pub y: i32
}

impl fmt::Display for Vec2i {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("X: {0}, Y: {1}", self.x, self.y))
    }
}

#[derive(Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3f {
    fn norm(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3f {
        Vec3f 
        {
            x: self.x * (1.0 / self.norm()),
            y: self.y * (1.0 / self.norm()),
            z: self.z * (1.0 / self.norm()),
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        Matrix {
            rows: 4,
            cols: 1,
            data: vec!(self.x, self.y, self.z, 1.)
        }
    }
}

impl fmt::Display for Vec3f {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("X: {0}, Y: {1}, Z: {2}", self.x, self.y, self.z))
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

pub struct Triangle3d {
    pub vertex_1: Vec3f,
    pub vertex_2: Vec3f,
    pub vertex_3: Vec3f
}

pub struct Triangle2d {
    pub point_1: Vec2i,
    pub point_2: Vec2i,
    pub point_3: Vec2i
}

/// Find the minimum x and y values for a given triangle (lower bounding box)
pub fn find_min(values: &Triangle2d) -> Vec2i {
    let mut min_x_val = -1;
    let mut min_y_val = -1;
    for val in [&values.point_1, &values.point_2, &values.point_3].iter() {
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
pub fn find_max(values: &Triangle2d) -> Vec2i {
    let mut max_x_val = -1; // WIDTH-1;
    let mut max_y_val = -1; //HEIGHT-1;
    for val in [&values.point_1, &values.point_2, &values.point_3].iter() {
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
pub fn cross(vec1: Vec3f, vec2: Vec3f) -> Vec3f {
    Vec3f {
        x: vec1.y * vec2.z - vec1.z * vec2.y,
        y: vec1.z * vec2.x - vec1.x * vec2.z,
        z: vec1.x * vec2.y - vec1.y * vec2.x
    }
}

/// Dot Product (Scalar) of two 3-component vectors
pub fn dot(vec1: Vec3f, vec2: Vec3f) -> f32 {
    (vec1.x * vec2.x) + (vec1.y * vec2.y) + (vec1.z * vec2.z)
}

/// Computes the barycentric coordinates of a given set of vectors and a point
pub fn barycentric(triangle: &Triangle2d, point: Vec2i) -> Vec3f {
    let u: Vec3f = cross(Vec3f {
        x: (triangle.point_3.x - triangle.point_1.x) as f32,
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