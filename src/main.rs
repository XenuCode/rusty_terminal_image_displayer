use argparse::{ArgumentParser, Store, StoreTrue};
use crate::image_to_symbol::generate_symbol_vec;

mod image_to_symbol;

fn main() {
    let mut verbose = false;
    let mut file_path:String = String::new();
    let arr: [u32;2];
    let mut grayscale:bool =false;
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Rusty Terminal Image Displayer aka rusty_terminal_image_displayer written in rust :D");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                        "Print Width and Height of final image");
                    ap.refer(&mut file_path)
        .add_option(&["-f","--file"], Store,
        "Path to file");
        //ap.refer(&mut dim)
        //.add_option(&["-d","--dimensions"], Store, "Dimension in which to display the image in format: width,height for example 16,16 \n dont use this option to display maximum possible size with \n correct aspect ratio");
        ap.refer(&mut grayscale)
            .add_option(&["-g", "--grayscale"], StoreTrue,
                        "Displays image in grayscale");
        ap.parse_args_or_exit();
    }
    //if !dim.is_empty() {
    //    let values: Vec<String> = dim.split(',')..collect();
    //     arr = [dim.split_at(dim.find(',').unwrap()).0.as_ptr() as u32,dim.split_at(dim.find(',').unwrap()).1.as_ptr() as u32];
    //}
    //else {
        match terminal_size::terminal_size() {
            None => {arr = [0,0]}
            Some(v) => {arr = [v.0.0 as u32,v.1.0 as u32]}
        }
   // }

    generate_symbol_vec(verbose,arr,file_path,grayscale);
}
