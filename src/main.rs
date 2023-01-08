use structopt::StructOpt;
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<RustCommand>,
}

#[derive(StructOpt)]
enum RustCommand {
    Memo {
        #[structopt(default_value = "memo")]
        name: String,
        #[structopt(default_value = "md")]
        extension: String ,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;

    match args.cmd {
        Some(RustCommand::Memo { name, extension }) => {
            let home_path = env::var("HOME").unwrap();
            let filepath = Path::new(&home_path).join(String::from("Desktop/") + &name + "." + &extension);
            let mut file = File::create(&filepath)?;
            file.write_all(String::from("").as_bytes())?;

            Command::new("open")
                    .arg(&filepath)
                    .output()
                    .expect("Failed to open file.");
        }
        None => println!("Set arguments."),
    }
    Ok(())
}
