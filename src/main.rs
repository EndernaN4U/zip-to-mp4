use std::io::prelude::*;
use std::fs::File;
use image::{ImageBuffer, Rgb};
use image;

fn buf_into_image(buffer: &[u8]){
    let len_sqrt = (buffer.len() as f64).sqrt() as u32;

    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(len_sqrt, len_sqrt);

    let mut x: usize = 0;
    for i in 0..len_sqrt{
        for j in 0..len_sqrt{
            let color = buffer[x];
            image.put_pixel(i,j, Rgb([color,color,color]));
            x+=1;
        }
    }
    image.save("./test/output.png").unwrap();
}

fn image_to_buf(path: String)->[u8;1024]{
    let buf = [0; 1024];
    let len_sqrt = (buf.len() as f64).sqrt() as u32;

    let image = image::open(path)
    .expect("Opening err")
    .into_rgb8();

    let mut x: usize = 0;
    let mut ret_buf = [0; 1024];
    for i in 0..len_sqrt{
        for j in 0..len_sqrt{
            let pixel = image.get_pixel(i,j);
            ret_buf[x] = pixel[0];
            x+=1;
        }
    }
    ret_buf
}

fn main() {
    let mut file = File::open("./test/test.zip")
    .expect("Opening file err");

    let mut buf = [0; 1024];

    file.read(&mut buf)
    .expect("Reading err");

    buf_into_image(&buf);

    let mut file2 = File::create("./test/test2.zip")
    .expect("Creating file err");

    let ret_buf  = image_to_buf(String::from("./test/output.png"));

    file2.write_all(&ret_buf)
    .expect("Writing err");
}
