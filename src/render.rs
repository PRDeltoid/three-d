use crate::utility::*;
use crate::math::*;

pub fn render(mesh: &tobj::Mesh, imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    //For every 3 vertex indicies belonging to a face...
    for face in mesh.indices.chunks(3) {
        //Build a vector containing a tuple of their (x, y, z) values
        let vertexes: Vec<Vec3f> = (0..3).map(|i| {
            let index = face[i] as usize;
            Vec3f {
                x: mesh.positions[index],
                y: mesh.positions[index + 1],
                z: mesh.positions[index + 2]
            }
        }).collect();

        //Render the triangle created by the 3 vertexes
        draw_triangle(imgbuf, vertexes);
    }
}

fn draw_triangle(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, vertexes: Vec<Vec3f>) {

    //Convert our vertexes to screen coordinates (x,y)
    let screen_vertexes = vertexes.iter().map(|v| {
        find_screen_coordinates(&v, buf.width(), buf.height() )
    }).collect();

    //Find our triangle's bounding box
    let bb = find_bounding_box(screen_vertexes);

    for x in (bb.min_values.x..bb.max_values.x) {
        for y in (bb.min_values.y..bb.max_values.y) {
            //For each pixel in the bounding box, compute barycentric coordinates
        }
    }
}

fn draw_line(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, pos1: (u32, u32), pos2: (u32, u32), color: image::Rgb<u8>) {

    //Extract positions into separate variables
    let mut x1 = pos1.0 as i32;
    let mut y1 = pos1.1 as i32;
    let mut x2 = pos2.0 as i32;
    let mut y2 = pos2.1 as i32;

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

//Converts (x,y,z) world coordinates to screen (pixel) coordinates (x,y)
fn find_screen_coordinates(vertex: &Vec3f, width: u32, height: u32) -> Vec2i {
    let x = ((vertex.x + 1.0) * (width as f32 / 2.0)) as i32;
    let y = ((vertex.y + 1.0) * (height as f32 / 2.0)) as i32;

    let x = std::cmp::min(x, width as i32 - 1);
    let y = std::cmp::max(y, height as i32 - 1);

    Vec2i {
        x,
        y
    }
}
