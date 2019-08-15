extern crate image;
extern crate tobj;

use std::path::Path;
use std::cmp;

static WIDTH: u32 = 1000;
static HEIGHT: u32 = 1000;

fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

struct Screen_Coordinate { 
    x: u32,
    y: u32
}

struct Bounding_Box {
    min_values: (u32, u32),
    max_values: (u32, u32)
}

//Finds the bounding box of a given triangle formed by 3 screen coordinates (x,y)
fn find_bounding_box(vertexes: Vec<Screen_Coordinate>) -> Bounding_Box {
    Bounding_Box {
        min_values: find_min(&vertexes),
        max_values: find_max(&vertexes)
    }


}

//Find the minimum x and y values for a given triangle (lower bounding box)
fn find_min(values: &Vec<Screen_Coordinate>) -> (u32, u32) {
    let mut min_x_val = 0;
    let mut min_y_val = 0;
    for val in values {
        if val.x < min_x_val {
            min_x_val = val.x;
        }

        if val.y < min_y_val {
            min_y_val = val.y;
        }
    }
    (min_x_val, min_y_val)
}

//Find the maximum x and y values for a given triangle (upper bounding box)
fn find_max(values: &Vec<Screen_Coordinate>) -> (u32, u32) {
    let mut max_x_val = 0; // WIDTH-1;
    let mut max_y_val = 0; //HEIGHT-1;
    for val in values {
        if val.x > max_x_val {
            max_x_val = val.x;
        }

        if val.y > max_y_val {
            max_y_val = val.y;
        }
    }
    (max_x_val, max_y_val)
}

//Converts (x,y,z) world coordinates to screen (pixel) coordinates (x,y)
fn find_screen_coords(vertex: &(f32, f32, f32)) -> Screen_Coordinate {
    let x = ((vertex.0 + 1.0)*(WIDTH as f32/2.0)) as u32;
    let y = ((vertex.1 + 1.0)*(HEIGHT as f32/2.0)) as u32;

    let x = std::cmp::min(x, WIDTH-1);
    let y = std::cmp::max(y, HEIGHT-1);

    Screen_Coordinate {
        x,
        y
    }

}

fn draw_triangle(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, vertexes: Vec<(f32, f32, f32)>) {

    //Convert our vertexes to screen coordinates (x,y)
    let screen_vertexes = vertexes.iter().map(|v| {
        find_screen_coords(v)
    }).collect();
        
    //Find our triangle's bounding box
    let bb = find_bounding_box(screen_vertexes);

    for x in (bb.min_values.0..bb.max_values.0) {
        for y in (bb.min_values.1..bb.max_values.1) {
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
    let steep = (x1-x2).abs() < (y1-y2).abs();
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
    let delta_err = (dy as f32/dx as f32).abs();


    println!("Outputting line from x: {}, y: {} to x: {}, y: {}", x1, y1, x2, y2);

    //Draw the line
    let mut error = 0.0;
    let mut y = y1;
    for x in x1 .. x2 {
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

fn render(mesh: &tobj::Mesh, imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    //For every 3 vertex indicies belonging to a face...
    for face in mesh.indices.chunks(3) {
        //Build a vector containing a tuple of their (x, y, z) values
        let vertexes: Vec<(f32, f32, f32)> = (0..3).map(|i| {
            let index = face[i] as usize;
             (mesh.positions[index], mesh.positions[index+1], mesh.positions[index+2])
        }).collect();

        //Render the triangle created by the 3 vertexes
        draw_triangle(imgbuf, vertexes);

    }

}

fn main() {

    //Load file 
    let file = tobj::load_obj(&Path::new("obj/african_head.obj"));

    let (model, _) = file.unwrap();
    let mesh = &model[0].mesh;

    //Set all pixels to black
    let mut imgbuf = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |_x,_y| {
        image::Rgb([0,0,0])
    });

    /*output orientation pixels
    imgbuf.put_pixel(1, 1, image::Rgb([0,255,0])); //green
    imgbuf.put_pixel(99, 1, image::Rgb([255,0,0])); //red
    imgbuf.put_pixel(99, 99, image::Rgb([0,0,255])); //blue*/

    render(mesh, &mut imgbuf);

    imgbuf.save("test.png").unwrap();
}

