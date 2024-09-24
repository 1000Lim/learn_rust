use image::{self, imageops, GenericImageView};
fn main(){
    let size = 128;
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} image_thumb imagefile", args[0]);
        return;
    }

    let infile = String::from(&args[1]);
    let file_name: Vec<&str> = infile.split('.').collect();
    let outfile = format!("{}_thumb.{}", file_name[0], file_name[1]);

    let mut img = image::open(&infile).expect("Could not open image file");
    let dim = img.dimensions();
    let w = if dim.0 > dim.1 {dim.1} else {dim.0};

    let img2 = imageops::crop(&mut img, (dim.0 - w)/2, (dim.1 -w)/2, w, w).to_image();
    let img3 = imageops::resize(&img2, size, size, image::imageops::FilterType::Lanczos3);
    img3.save(&outfile).expect("Could not save image file");


}