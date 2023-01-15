use structopt::StructOpt;
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "memo")]
    name: String,
    #[structopt(default_value = "md")]
    extension: String ,
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;
    let home_path = env::var("HOME").unwrap();
    let filepath = Path::new(&home_path).join(String::from("Desktop/") + &args.name + "." + &args.extension);
    let mut file = File::create(&filepath)?;
    file.write_all(String::from("").as_bytes())?;

    Command::new("open")
            .arg(&filepath)
            .output()
            .expect("Failed to open file.");
    Ok(())
}
