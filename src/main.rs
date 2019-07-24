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
        //Connect each vertex to its next neighbor
        for i in 0..3 {
            let v1 = (mesh.positions[(face[i] as usize)*3], mesh.positions[(face[i] as usize)*3+1]);
            let v2 = (mesh.positions[(face[(i+1)%3] as usize)*3], mesh.positions[(face[(i+1)%3] as usize)*3+1]);


            println!("{:?} {:?}", v1, v2);
            let x0 = ((v1.0 + 1.0)*(WIDTH as f32)/2.0) as u32; 
            let y0 = ((v1.1 + 1.0)*(HEIGHT as f32)/2.0) as u32; 
            let x1 = ((v2.0 + 1.0)*(WIDTH as f32)/2.0) as u32; 
            let y1 = ((v2.1 + 1.0)*(HEIGHT as f32)/2.0) as u32; 

            //Make sure the values don't hit the bounds of our picture
            let x0 = std::cmp::min(x0, WIDTH-1);
            let y0 = std::cmp::min(y0, HEIGHT-1);
            let x1 = std::cmp::min(x1, WIDTH-1);
            let y1 = std::cmp::min(y1, HEIGHT-1);
            draw_line(imgbuf, (x0, y0), (x1, y1), image::Rgb([255, 255, 255]));
        }

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

    imgbuf.save("test.ppm").unwrap();
}

