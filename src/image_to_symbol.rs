use image::GenericImageView;
use image::imageops::FilterType;
use colored::Colorize;
pub fn generate_symbol_vec(verbose:bool,size: [u32;2],file_path:String,grayscale:bool){
    let mut img = image::open(file_path).expect("File not accessible/ not found"); //"/home/xenu/testIMG/img.jpg"
    if size[0] == 0 || size[1] == 0{
    }
    else {
        img = img.resize(size[0]/2, size[1],FilterType::Triangle );
    }
    if grayscale {
        img =img.grayscale();
    }

    let width =img.width();
    let height = img.height();
    print!("{}[2J", 27 as char); //only clears screen
    //print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //clears and changes cursor position
    for y in 0..height
    {
        for x in 0..width
        {
            let slice =img.get_pixel(x,y).0;
            print!("{}", "  ".on_truecolor(slice.get(0).unwrap().clone(), slice.get(1).unwrap().clone(), slice.get(2).unwrap().clone()));
        }
        print!("\n");
    }
    if verbose {
        println!("H:{} W:{}",img.height(),img.width());
    }
}