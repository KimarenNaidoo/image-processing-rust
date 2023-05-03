use std::io;
use image::DynamicImage;
use image_processing_app::{blur, invert, brighten, crop, rotate, grayscale, print_usage_and_exit};

fn main() -> io::Result<()> {
    println!("\n <<< Image Processing >>> \n");
    println!("Enter processing prompts or enter 'help' to view the prompts");
    let mut user_input= String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_n) => {
            let args: Vec<String> = user_input.split_whitespace().map(str::to_string).collect();
            process(args);
        }
        Err(error) => println!("error: {error}"),
    }

    Ok(())
}

fn process(mut args: Vec<String>) {
    let initial_value = args.remove(0);

    if args.is_empty() || initial_value == "help" {
        print_usage_and_exit();
    }

    let mut in_file: DynamicImage = image::open(initial_value).expect("Failed to open INFILE {}.");
    let out_file = args.remove(0);

    loop {
        if args.is_empty() {
            println!("Processing is complete. See file {} to view results.", out_file);
            break;
        }

        let process = args.remove(0);

        match process.as_str() {
            "blur" => {
                let sigma: f32 = args.remove(0).parse().expect("Failed to parse number");
                in_file = blur(sigma, in_file);
            },
            "brighten" => {
                let value: i32 = args.remove(0).parse().expect("Failed to parse number");
                
                in_file = brighten(value, in_file); 
            },
            "crop" => {
                let x: u32 = args.remove(0).parse().expect("Failed to parse number");
                let y: u32 = args.remove(0).parse().expect("Failed to parse number");
                let width: u32 = args.remove(0).parse().expect("Failed to parse number");
                let height: u32 = args.remove(0).parse().expect("Failed to parse number");

                in_file = crop(x, y, width, height, in_file); 
            },
            "rotate" => {
                let degree: i32 = args.remove(0).parse().expect("Failed to parse number");

                in_file = rotate(degree, in_file);
            },
            "invert" => {
                in_file = invert(in_file);
            },
            "grayscale" => {
                in_file = grayscale(in_file);
            },
            _ => {
                // last option (default)
            }
        }
    }

    in_file.save(out_file).expect("Failed to save to out file");
}
