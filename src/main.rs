use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "wewa")]
struct Arguments {
    #[clap(value_parser)]
    file_name: Option<String>,
}

fn main() {
    let arguments = Arguments::parse();

    if let Some(file_name) = arguments.file_name.as_deref() {
        print!("File name is \"{}\"\n", file_name);
    }
}
