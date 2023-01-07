use structopt::StructOpt;
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::Path;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Memo { name: String, extension: String },
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;

    match args.cmd {
        Some(Command::Memo { name, extension }) => {
            let home_path = env::var("HOME").unwrap();
            let filepath = Path::new(&home_path).join(String::from("Desktop/") + &name + "." + &extension);
            let mut file = File::create(&filepath)?;
            file.write_all(String::from("").as_bytes())?;
        }
        None => println!("Set arguments."),
    }
    Ok(())
}
