use std::io::prelude::*;
use std::fs::File;

fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
        std::io::copy(&mut file, &mut std::io::stdout()).expect("Copy error");
    }

    Ok(())
}
fn main() {
    let mut file = File::open("./test/test.zip").expect("Blagam no");
    let mut buf = [0; 1024];
    let err = file.read(&mut buf);
    err.expect("Reading err");

    let mut file2 = File::create("./test/test2.zip").expect("pls");
    let err2 = file2.write_all(&buf);
    err2.expect("Writeing err");
    let _ = list_zip_contents(file2);
}
