use clap::{Arg, command, Command, value_parser};
use image_tools::{renderer, enums::Direction};

fn main() {
    get_arguments();

}

fn get_arguments() {
    // default arguments
    let source_image: Arg = Arg::new("source_image")
        .short('s')
        .long("source")
        .aliases(["source-image", "source_image", "sourceImage", "simage"])
        .required(true)
        .help("Source image");

    let target_image: Arg = Arg::new("target_image")
        .short('t')
        .long("target")
        .aliases(["target-image", "target_image", "targetImage", "starget"])
        .required(true)
        .help("Image that will be created with the desired effect");

    let width: Arg = Arg::new("width")
        .long("width")
        .required(true)
        .value_parser(value_parser!(u32))
        .help("Select a number above 0.");


    let height: Arg = Arg::new("height")
        .long("height")
        .required(true)
        .value_parser(value_parser!(u32))
        .help("Select a number above 0.");


    // commands

    let commands = command!()
        .about("Select which action you want to apply to a image")
        .subcommand(Command::new("blur")
            .arg(&source_image)
            .arg(&target_image)
            .arg(Arg::new("blur_value")
                .short('b')
                .long("blur-value")
                .aliases(["value", "blur_value", "blurValue"])
                .required(true)
                .value_parser(value_parser!(f32))
                .help("A float value that represents the amount of blur applied.")
            )
        )
        .subcommand(Command::new("rotate")
            .arg(&source_image)
            .arg(&target_image)
            .arg(Arg::new("direction")
                .short('d')
                .long("direction")
                .required(true)
                .help("Select Right or Left.")
            )
            .arg(Arg::new("rotation")
                .short('r')
                .long("rotation-value")
                .aliases(["value", "rotation_value", "rotationValue"])
                .required(true)
                .value_parser(value_parser!(u32))
                .help("Select 90, 180 or 270.")
            )
        )
        .subcommand(Command::new("resize")
            .arg(&source_image)
            .arg(&target_image)
            .arg(&width)
            .arg(&height)
        )
        .get_matches();

    match commands.subcommand() {
        Some((command_name, command_args)) => {
            let source = command_args.get_one::<String>("source_image").unwrap();
            let target = command_args.get_one::<String>("target_image").unwrap();

            match command_name {
                "blur" => {
                    let blur = command_args.get_one::<f32>("blur_value").unwrap();
                    renderer::blur(&source, &target, *blur);
                },
                "rotate" => {
                    let direction = match command_args.get_one::<String>("direction").unwrap().to_lowercase().as_str() {
                        "left" => Direction::Left,
                        "right" => Direction::Right,
                        _ => panic!("Invalid value. Select Right or Left."),
                    };

                    let rotation = match command_args.get_one::<u32>("rotation").unwrap() {
                        90 => 90,
                        180 => 90,
                        270 => 270,
                        _ => panic!("Invalid value. Select 90, 180 or 270."),
                    };

                    renderer::rotate(source, target, direction, rotation);
                },
                "resize" => {
                    let width = command_args.get_one::<u32>("width").unwrap();
                    let height = command_args.get_one::<u32>("height").unwrap();
                    renderer::resize(source, target, &width, &height);
                },
                _ => println!("Error..."),
            }
        },
        None => println!("Error..."),
    }
}