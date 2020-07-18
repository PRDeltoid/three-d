use crate::utility::*;
use crate::math::*;

use image::{RgbImage, ImageBuffer, imageops, Pixel, RGB};
use tobj::{Model, LoadError, Material};

struct TriangleTexture {
    pub point_1: Vec2f,
    pub point_2: Vec2f,
    pub point_3: Vec2f
}

/// Render a flat image from a OBJ mesh
pub fn render(object: Result<(Vec<Model>, Vec<Material>), LoadError>, texture: &RgbImage, width: u32, height: u32, filename: &str) {
    //Unpack the loaded 3d object
    let (model, _) = object.unwrap();

    //Unpack the first models mesh (we will add multi-model rendering later)
    let mesh = &model[0].mesh;
    //let texcoords = &mesh.texcoords;

    //Set all background pixels to black
    let mut imgbuf: RgbImage = ImageBuffer::from_fn(width, height, |_x, _y| {
        image::Rgb([0, 0, 0])
    });

    //draw_orientation_marks(&mut imgbuf);
    //build a z-buffer to make sure we arn't drawing over pixels
    let mut z_buffer: Vec<i32> = vec![-1; (width * height) as usize];

    // Render each triangle
    // For each 3 indices representing the vertexes of a triangle face
    for vertex_indexes in mesh.indices.chunks(3) {
        //println!("Face indexes: {}/{}/{}", vertex_indexes[0], vertex_indexes[1], vertex_indexes[2]);

        //Get the exact [x,y,z] position for each vertex in face using the indexes
        let vertexes: Vec<Vec3f> = (0..3).map(|i| {
            let vertex_1_index = vertex_indexes[i] as usize * 3;
            let vertex_2_index = vertex_indexes[i] as usize * 3 + 1;
            let vertex_3_index = vertex_indexes[i] as usize * 3 + 2;
            //println!("Vertex {}: {} ({}), {} ({}),  {} ({})", vertex_indexes[i], vertex_1_index, mesh.positions[vertex_1_index], vertex_2_index, mesh.positions[vertex_2_index], vertex_3_index, mesh.positions[vertex_3_index]);

            //mesh.positions if flattened, so each index points to the start of 3 vertexes of the given face in the positions vector.
            //  Because it is flattened, we can get the other two vertexes of the face by simply adding 1 and 2 to the index
            Vec3f {
                x: mesh.positions[vertex_1_index],
                y: mesh.positions[vertex_2_index],
                z: mesh.positions[vertex_3_index]
            }
        }).collect();

        let tex_vertexes: Vec<Vec2f> = (0..3).map(|i| {
            let vertex_1_index = vertex_indexes[i] as usize * 2;
            let vertex_2_index = vertex_indexes[i] as usize * 2 + 1;

            Vec2f {
                x: mesh.texcoords[vertex_1_index],
                y: mesh.texcoords[vertex_2_index]
            }
        }).collect();

        //Construct a triangle from the [x,y,z] vertexes
        let triangle = Triangle3d {
            vertex_1: vertexes[0],
            vertex_2: vertexes[1],
            vertex_3: vertexes[2]
        };

        let texture_triangle = TriangleTexture {
            point_1: tex_vertexes[0],
            point_2: tex_vertexes[1],
            point_3: tex_vertexes[2]
        };

        draw_triangle(&mut imgbuf, triangle, &mut z_buffer, &texture, texture_triangle);
    }

    // Flip the image
    let imgbuf = imageops::rotate180(&imgbuf);
    // Save the image
    imgbuf.save(filename).unwrap();
}

/// Draw a triangle on the image buffer
fn draw_triangle(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, triangle: Triangle3d, z_buffer: &mut Vec<i32>, texture: &RgbImage, triangle_texture: TriangleTexture) {

    //println!("Printing triangle [P1: ({0}), P2: ({1}), P3: ({2})]", triangle.point_1, triangle.point_2, triangle.point_3);
    
    //Construct a 2D triangle with screen coordinates based on our 3D triangle
    let triangle_2d = Triangle2d {
        point_1: find_screen_coordinates(&triangle.vertex_1, buf.width(), buf.height()),
        point_2: find_screen_coordinates(&triangle.vertex_2, buf.width(), buf.height()),
        point_3: find_screen_coordinates(&triangle.vertex_3, buf.width(), buf.height())
    };
    
    //Find our 2D triangle's bounding box
    let bb = find_bounding_box(&triangle_2d);
    
    //We need to determine the intensity with which light hits the surface to determine it's brightness
    // The closer to parallel the triangle normal is to the light vector, the brighter the face will be
    //First, normalize the cross produce of the triangle to get the triangle normal vector
    let triangle_normal = cross(triangle.vertex_3 - triangle.vertex_1, triangle.vertex_2 - triangle.vertex_1).normalize();
    
    //Second, construct a hardcoded light coming from the viewport
    let light_direction = Vec3f { x: 0.0, y: 0.0, z: -1.0 };
    
    //Third, determine the scalar/length of the triangle normal vector and light vector
    let intensity = dot(triangle_normal, light_direction);

    //Negative intensity means the light is coming from behind the face and therefore the face should not be drawn (it would be pitch black otherwise)
    if intensity < 0.0 {
        return;
    }
    
    //Default color is black for now, giving the image a greyscale appearance 
    //let color = [255.0,255.0,255.0];
    //color = [rng.gen::<u8>() % 255, rng.gen::<u8>() % 255, rng.gen::<u8>() % 255];

    // For each point in the triangle's bounding box, determine if it should be drawn to the image
    for x in bb.min_values.x..bb.max_values.x {
        for y in bb.min_values.y..bb.max_values.y {
            // Compute the barycentric coordinates of the current point in the bounding box
            let vec = barycentric(&triangle_2d, Vec2i {x,y});

            // if the barycentric coordinates contain any negative values, the point lies outside of the triangle and should not be drawn on the screen
            if vec.x < 0.0 || vec.y < 0.0 || vec.z < 0.0 { continue; }

            // Determine the color of the pixel based on the texture map
            let color = {
                // Calculate the exact x,y location of the pixel in the triangle relative to where we are on the 3D triangle
                let weighted_x = (triangle_texture.point_1.x * vec.x) + (triangle_texture.point_2.x * vec.y) + (triangle_texture.point_3.x * vec.z);
                let weighted_y = (triangle_texture.point_1.y * vec.x) + (triangle_texture.point_2.y * vec.y) + (triangle_texture.point_3.y * vec.z);
                let tex_coords: Vec2i = Vec2i {
                    x: (weighted_x*texture.width() as f32) as i32,
                    y: (weighted_y*texture.height() as f32) as i32
                };
                //println!("{} {}", tex_coords.x, tex_coords.y);
                let pixel = texture.get_pixel(tex_coords.x as u32, tex_coords.y as u32);
                [pixel[0] as u8, 
                pixel[1] as u8,
                pixel[2] as u8]
            };

            // compute the z location of the current pixel we are drawing via interpolation 
            //   between the face's vertexes z values and the barycentric coordinates of the point being drawn
            let z: i32 = {
                [triangle.vertex_1.z * vec.x, 
                 triangle.vertex_2.z * vec.y,
                 triangle.vertex_3.z * vec.z].iter().sum::<f32>() as i32
            };

            // Only draw this pixel if it is above all other currently drawn pixels in this position (painters algorithm)
            if z_buffer[(x+y*buf.width() as i32) as usize] < z  {
                // Set the new highest z-level for our point to our calculated z-level
                z_buffer[(x+y*buf.width() as i32) as usize] = z;
                // Determine the color based on the intensity of light hitting the face 
                let color: [u8; 3] = [(intensity * color[0] as f32) as u8, (intensity * color[1] as f32) as u8, (intensity * color[2] as f32) as u8];
                // Draw the pixel on the screen
                buf.put_pixel(x as u32, y as u32, image::Rgb { data:color });
            }
        }
    }
}

/// Draw 3 different colored pixels in each corner to test orientation
#[allow(dead_code)]
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
