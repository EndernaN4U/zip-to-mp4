mod img_buf;

fn main() {
    let input = String::from("./test/test.zip");
    let img_output = String::from("./test/output.png");
    let output = String::from("./test/output.zip");
    
    let input_buffer = img_buf::ImgBuf::get_buf_of_file::<1024>(input);

    img_buf::ImgBuf::buf_into_image(&input_buffer, img_output.clone());

    let image_buffer = img_buf::ImgBuf::image_into_buf::<1024>(img_output.clone());
    
    img_buf::ImgBuf::save_file_with_buf(&image_buffer, output);
}
