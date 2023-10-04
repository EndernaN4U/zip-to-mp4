use std::io::prelude::*;
use std::fs::File;
use image::{ImageBuffer, Rgb};


fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
        std::io::copy(&mut file, &mut std::io::stdout()).expect("Copy error");
    }

    Ok(())
}

fn buf_into_image(buffer: &[u8]){
    let len_sqrt = (buffer.len() as f64).sqrt() as u32 ;
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(len_sqrt, len_sqrt);
    let mut x: usize = 0;
    for i in 0..len_sqrt{
        for j in 0..len_sqrt{
            let color = buffer[x];
            image.put_pixel(i,j, Rgb([color,color,color]));
            x+=1;
        }
    }
    image.save("./test/output.jpg").unwrap();
}

fn image_to_buf(mut image: File)->[u8;1024]{
    let mut buf = [0; 1024];
    let err = image.read(&mut buf);
    let len_sqrt = (buf.len() as f64).sqrt() as u32 / 2;
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(len_sqrt, len_sqrt, Vec::from(buf)).unwrap();

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
    let mut file = File::open("./test/test.zip").expect("Blagam no");
    let mut buf = [0; 1024];
    let err = file.read(&mut buf);
    println!("{:?}", buf);
    err.expect("Reading err");

    buf_into_image(&buf);

    let mut image = File::open("./test/output.jpg").expect("Blagam no");
    let mut file2 = File::create("./test/test2.zip").expect("pls");
    let mut ret_buf  = image_to_buf(image);
    println!("{:?}", ret_buf);
    let err2 = file2.write_all(&ret_buf);
    err2.expect("Writeing err");
}
