use std::io::prelude::*;
use std::fs::File;
use image::{ImageBuffer, Rgb};

pub struct ImgBuf;
impl ImgBuf{
    pub fn get_buf_of_file<const LENGTH: usize>(path: String)->[u8; LENGTH]{
        let mut file = File::open(path)
        .expect("Opening file err");
        
        let mut buf = [0; LENGTH];

        file.read(&mut buf)
        .expect("Reading err");

        buf
    }

    pub fn save_file_with_buf(buffer: &[u8], out_path: String){
        let mut file = File::create(out_path)
        .expect("Creating file err");

        file.write_all(buffer)
        .expect("Writing err");
    }

    pub fn buf_into_image(buffer: &[u8], out_path: String){
        let len_sqrt = ((buffer.len() / 3) as f64).sqrt().ceil() as u32;
    
        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(len_sqrt, len_sqrt);
    
        let mut x: usize = 0;
        'outer: for i in 0..len_sqrt{
            for j in 0..len_sqrt{
                if buffer.len() <= x + 2{break 'outer;}
                image.put_pixel(i,j, Rgb([buffer[x],buffer[x+1],buffer[x+2]]));   
                x+=3;
            }
        }

        image.save(out_path).unwrap();
    }

    pub fn image_into_buf<const LENGTH: usize>(path: String)->[u8; LENGTH]{
        let mut ret_buf = [0; LENGTH];
        let len_sqrt = ((ret_buf.len() / 3) as f64).sqrt().ceil() as u32;
    
        let image = image::open(path)
        .expect("Opening err")
        .into_rgb8();
    
        let mut x: usize = 0;
        'outer: for i in 0..len_sqrt{
            for j in 0..len_sqrt{
                let pixel = image.get_pixel(i,j);
                for k in 0..3{
                    if ret_buf.len() <= x{break 'outer;}
                    ret_buf[x] = pixel[k];
                    x+=1;
                }  
            }
        }
        ret_buf
    }
}