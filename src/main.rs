mod utility;//Misc helper functions
mod math;   //Math-related functions
mod render; //Rendering-related functions

extern crate image; //Image export library
extern crate tobj;  //TObj file loader library

use std::path::Path;  //Used for locate TObj file in filesystem
use std::cmp;

static WIDTH: u32 = 1000;
static HEIGHT: u32 = 1000;

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

    render::render(mesh, &mut imgbuf);

    imgbuf.save("test.png").unwrap();
}

