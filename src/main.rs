use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "wewa")]
struct Arguments {
    #[clap(value_parser)]
    file_name: Option<String>,

    #[clap(short, long, action)]
    all: bool,
}

fn main() {
    let arguments = Arguments::parse();

    if arguments.all {
        println!("All files are selected!");
    }

    if let Some(file_name) = arguments.file_name.as_deref() {
        println!("File name is \"{}\"", file_name);
    }
}
