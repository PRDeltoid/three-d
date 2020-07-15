mod utility;//Misc helper functions
mod math;   //Math-related functions
mod render; //Rendering-related functions

extern crate image; //Image export library
extern crate tobj;  //TObj file loader library

use std::path::Path;  //Used for locate TObj file in filesystem

static WIDTH: u32 = 1000;
static HEIGHT: u32 = 1000;

fn main() {

    //Load file 
    let object = tobj::load_obj(Path::new("obj/african_head.obj"), true);
    //let object = tobj::load_obj(&Path::new("obj/teapot.obj"), true);

    render::render(object, WIDTH, HEIGHT, "test.png");
}

