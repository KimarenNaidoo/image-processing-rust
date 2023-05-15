use image::DynamicImage;

pub fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("Enter the 'INFILE.png OUTFILE.png' and followed by the commands you want to run.");
    println!("You can chain the commands in any order.");
    println!("'blur' takes one value.");
    println!("'brighten' takes one value.");
    println!("'crop' takes four values.");
    println!("'rotate' takes one value (90, 180, 270).");
    println!("'invert' takes no values.");
    println!("'invert' takes no values.");
    println!("For example: 'infile.png outfile.png blur 10 invert ...'");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

pub fn blur(sigma: f32, infile: DynamicImage) -> DynamicImage {
    infile.blur(sigma)
}

pub fn brighten(value: i32, infile: DynamicImage) -> DynamicImage {
    // Positive numbers brighten the image
    // Negative numbers darken it.  It returns a new image.
    infile.brighten(value)
}

pub fn crop(x: u32, y: u32, width: u32, height: u32, mut infile: DynamicImage) -> DynamicImage {
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    infile.crop(x, y, width, height)
}

pub fn rotate(rotation: i32, infile: DynamicImage) -> DynamicImage {
    match rotation {
        90 => {
            infile.rotate90()
        },
        180 => {
            infile.rotate180()
        },
        270 => {
            infile.rotate270()
        },
        _ => {
            infile
        }
    }
}

pub fn invert(mut infile: DynamicImage)-> DynamicImage {
    // .invert() takes no arguments and converts the image in-place
    infile.invert();
    infile
}

pub fn grayscale(infile: DynamicImage) -> DynamicImage {
    // .grayscale() takes no arguments. It returns a new image.
    infile.grayscale()
}