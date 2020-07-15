use crate::utility::*;
use crate::math::*;

use rand::prelude::*;
use image::{RgbImage, ImageBuffer};

/// Render a flat image from a OBJ mesh
pub fn render(object: &tobj::LoadResult, width: u32, height: u32, filename: &str) {
    //Unpack the loaded object
    let (model, _) = object.unwrap();
    let mesh = &model[0].mesh;

    //Set all pixels to black
    let mut imgbuf: RgbImage = ImageBuffer::from_fn(width, height, |_x,_y| {
        image::Rgb([0,0,0])
    });

    //draw_orientation_marks(&mut imgbuf);

    //For every 3 vertex indices belonging to a face...
    for face in mesh.indices.chunks(3) {
        println!("Face: {} {} {}", face[0], face[1], face[2]);
        //Build a vector containing a collection of all vertexes (x, y, z) values
        let vertexes: Vec<Vec3f> = (0..3).map(|i| {
            let index = face[i] as usize;
            Vec3f {
                x: mesh.positions[index],
                y: mesh.positions[index + 1],
                z: mesh.positions[index + 2]
            }
        }).collect();

        let triangle = Triangle {
            point_1: find_screen_coordinates(&vertexes[0],imgbuf.width(), imgbuf.height()),
            point_2: find_screen_coordinates(&vertexes[1],imgbuf.width(), imgbuf.height()),
            point_3: find_screen_coordinates(&vertexes[2],imgbuf.width(), imgbuf.height())
        };

        //Render the triangle created by the 3 vertexes
        draw_triangle(&mut imgbuf, triangle);
    }

    //Save the image
    imgbuf.save(filename).unwrap();
}

/// Draw a triangle on the image buffer
fn draw_triangle(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, triangle: Triangle) { //vertexes: Vec<Vec3f>) {

    let mut rng = rand::thread_rng();
    if triangle.point_1.x < 0 || triangle.point_1.y < 0 { return; }
    if triangle.point_2.x < 0 || triangle.point_2.y < 0 { return; }
    if triangle.point_3.x < 0 || triangle.point_3.y < 0 { return; }
    if triangle.point_1.x > 999  || triangle.point_1.y > 999 { return; }
    if triangle.point_2.x > 999  || triangle.point_2.y > 999 { return; }
    if triangle.point_3.x > 999  || triangle.point_3.y > 999 { return; }
    //println!("Printing triangle [P1: ({0}), P2: ({1}), P3: ({2})]", triangle.point_1, triangle.point_2, triangle.point_3);
    //Find our triangle's bounding box
    let bb = find_bounding_box(&triangle);

    let color: [u8 ; 3] = [rng.gen::<u8>() % 255, rng.gen::<u8>() % 255, rng.gen::<u8>() % 255];

    for x in bb.min_values.x..bb.max_values.x {
        for y in bb.min_values.y..bb.max_values.y {
            //For each pixel in the bounding box, compute barycentric coordinates
            let vec = barycentric(&triangle, Vec2i {x,y});
            if vec.x < 0.0 || vec.y < 0.0 || vec.z < 0.0 { continue; }
            buf.put_pixel(x as u32, y as u32, image::Rgb { data: color });
        }
    }
}

/// Draw 3 different colored pixels in each corner to test orientation
fn draw_orientation_marks(imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    // output orientation pixels
    imgbuf.put_pixel(1, 1, image::Rgb([0,255,0]));      //green
    imgbuf.put_pixel(99, 1, image::Rgb([255,0,0]));     //red
    imgbuf.put_pixel(99, 99, image::Rgb([0,0,255]));    //blue
}

/// Render a wireframe mesh of a given OBJ file
#[allow(dead_code)]
pub fn render_mesh(_mesh: &tobj::Mesh, _imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    //TODO
}

/// Draws a line between two positions as a given color
#[allow(dead_code)]
fn draw_line(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, pos1: Vec2i, pos2: Vec2i, color: image::Rgb<u8>) {

    //Extract positions into separate variables
    let mut x1 = pos1.x as i32;
    let mut y1 = pos1.y as i32;
    let mut x2 = pos2.x as i32;
    let mut y2 = pos2.y as i32;

    //Determine if the line is steep (rises more than it runs)
    //Transpose points if the line is steep (we'll draw like X is Y and Y is X)
    let steep = (x1 - x2).abs() < (y1 - y2).abs();
    if steep {
        println!("Line is steep");
        swap(&mut x1, &mut y1);
        swap(&mut x2, &mut y2);
    }

    //Swap so we always draw from left to right
    if x1 > x2 {
        swap(&mut x1, &mut x2);
        swap(&mut y1, &mut y2);
    }

    //Calculate the slope
    let dy = y2 - y1;
    let dx = x2 - x1;

    //Determine direction in which line moves on the y axis (up or down)
    //If our y2 is greater than y1, we know we are drawing downwards and must increase Y
    //Otherwise, we are drawing upwards, and we want to decrease Y
    let sign: i32 = if y2 > y1 { 1 } else { -1 };

    //Amount away from "perfect" line we'll move with every step in the X direction
    let delta_err = (dy as f32 / dx as f32).abs();


    println!("Outputting line from x: {}, y: {} to x: {}, y: {}", x1, y1, x2, y2);

    //Draw the line
    let mut error = 0.0;
    let mut y = y1;
    for x in x1..x2 {
        //Place a pixel
        if !steep {
            buf.put_pixel(x as u32, y as u32, color);
        } else {
            //If it's steep, we must un-transpose our points
            buf.put_pixel(y as u32, x as u32, color);
        }

        //If we are too far from the ideal line, move our plotter in the Y direction
        error += delta_err;
        if error >= 0.5 {
            y = y + sign;
            error -= 1.0;
        }
    }
}
