extern crate image;

fn draw_line(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, mut pos1: (u32, u32), mut pos2: (u32, u32), color: image::Rgb<u8>) {

    if pos1.0 < pos2.0 {
        let tmp = pos1;
        pos1 = pos2;
        pos2 = tmp;
    }
    let (x1, y1) = pos1;
    let (x2, y2) = pos2;
    println!("Outputting line from x: {}, y: {} to x: {}, y: {}", x1, y1, x2, y2);

    let slope = (y1 as i32 - y2 as i32)/(x1 as i32 - x2 as i32);

    println!("Slope: {}", slope);
    let mut step: u32 = 1;
    if slope == 0 {
        step = 0;
    } else if slope > 1 {
        step = slope as u32;
    } else if slope < 1 {
        step = (slope as f32).powi(-1).round() as u32;
    }
    println!("Step: {}", step);
    


    let mut y = y1;
    for x in x2 .. x1 {
        for _i in 0 .. step {
            y+=1;
            //println!("Placing pixel at x: {}, y: {}", x, y);
            buf.put_pixel(x, y, color); //image::Rgb([255, 255, 255])); 
        }

    }

}

fn main() {
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

    //draw a test triangle
    draw_line(&mut imgbuf, (20,20), (50, 50), image::Rgb([0,255,0])); //green
    draw_line(&mut imgbuf, (50,50), (60, 20), image::Rgb([255,0,0])); //red
    draw_line(&mut imgbuf, (60,20), (20, 20), image::Rgb([0,0,255])); //blue

    imgbuf.save("test.ppm").unwrap();
}

