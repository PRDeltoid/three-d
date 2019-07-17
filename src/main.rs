extern crate image;
extern crate tobj;

use std::path::Path;
use std::sync::Arc;

struct Face {
    vertexes: Vec<(u32, u32, u32)>,
}

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
    let faces: Vec<(u32, u32, u32)> = mesh.indices.chunks(3).map(|i| {
        (i[0], i[1], i[2])
    }).collect();

    println!("{:?}", &faces[0]);


    let triangles: Vec<(f32, f32, f32)> = faces.iter().map(|i| {
        (mesh.positions[i.0 as usize], mesh.positions[i.1 as usize], mesh.positions[i.2 as usize])
    }).collect();

    println!("{:?}", &triangles[0..3]);

    for triangle in triangles {

    }


    /*for face in model.mesh.indices {
        let v1 = (model.mesh.positions[(face as usize)*3], model.mesh.positions[(face as usize)*3+1]); //, model.mesh.positions[(face as usize)*3+2]);

        //draw_line(&mut imgbuf, v1, v2, image::Rgb([255, 255, 255]))

    }*/

}

fn main() {

    //Load file 
    let file = tobj::load_obj(&Path::new("obj/african_head.obj"));

    let (model, _) = file.unwrap();
    let mesh = &model[0].mesh;

    let x = 100;
    let y = 100;
    //Set all pixels to black
    let mut imgbuf = image::ImageBuffer::from_fn(x, y, |_x,_y| {
        image::Rgb([0,0,0])
    });

    //output orientation pixels
    imgbuf.put_pixel(1, 1, image::Rgb([0,255,0])); //green
    imgbuf.put_pixel(99, 1, image::Rgb([255,0,0])); //red
    imgbuf.put_pixel(99, 99, image::Rgb([0,0,255])); //blue

    render(mesh, &mut imgbuf);

    /*draw a test triangle
    draw_line(&mut imgbuf, (25,25), (50, 50), image::Rgb([0,255,0])); //green
    draw_line(&mut imgbuf, (50,50), (75, 25), image::Rgb([255,0,0])); //red
    draw_line(&mut imgbuf, (75,25), (25, 25), image::Rgb([0,0,255])); //blue
    */

    imgbuf.save("test.ppm").unwrap();
}

