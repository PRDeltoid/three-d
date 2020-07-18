mod utility;//Misc helper functions
mod math;   //Math-related functions
mod render; //Rendering-related functions

extern crate image; //Image export library
extern crate tobj;  //TObj file loader library

use std::path::Path;  //Used for locate TObj file in filesystem

static WIDTH: u32 = 500;
static HEIGHT: u32 = 500;

fn main() {

    //Load file 
    let object = tobj::load_obj(Path::new("obj/african_head.obj"), true);
    let texture = image::open(Path::new("tex/african_head_diffuse.tga")).unwrap();
    //let object = tobj::load_obj(&Path::new("obj/teapot.obj"), true);
    //let object = tobj::load_obj(&Path::new("obj/cow.obj"), true);

    render::render(object, texture.as_rgb8().unwrap(), WIDTH, HEIGHT, "out/test.png");
}

