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

fn buf_to_image(buffer: &[u8]){
    println!("{:?}", buffer);
    let len_sqrt = (buffer.len() as f64).sqrt() as u32 + 1;
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(len_sqrt, len_sqrt);

    let mut i = 0;
    let mut j = 0;

    for buff in buffer{
        image.put_pixel(i, j, Rgb([*buff,*buff,*buff]));
        i += 1;
        if i >= len_sqrt{
            i = 0;
            j += 1;
        }
    }

    image.save("./test/output.png").unwrap();
}
fn main() {
    let mut file = File::open("./test/test.zip").expect("Blagam no");
    let mut buf = [0; 1024];
    let err = file.read(&mut buf);
    err.expect("Reading err");

    // let mut file2 = File::create("./test/test2.zip").expect("pls");
    // let err2 = file2.write_all(&buf);
    // err2.expect("Writeing err");
    // let _ = list_zip_contents(file2);

    buf_to_image(&mut buf);
}
