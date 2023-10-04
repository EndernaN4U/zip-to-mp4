use zip::read::ZipArchive;
use std::io::prelude::*;
use std::fs::File;

fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
        std::io::copy(&mut file, &mut std::io::stdout());
    }

    Ok(())
}
fn main() {
    let mut file = File::open("./test/test.zip").expect("Blagam no");

    list_zip_contents(file);
    
}
