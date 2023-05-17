extern crate clap;

use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "2")]
    duration: u64,

    #[clap(short, long, default_value = "true")]
    print_delimiter: bool,

    #[clap(trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let cmd_string = &args.command.join(" ");
    let cmd_name = &args.command[0];
    loop {
        if args.print_delimiter {
            let now = chrono::Local::now().format("%y-%m-%d %H:%M:%S");
            println!("{:}", now);
            println!("{:}", cmd_string);
        }

        let mut child = std::process::Command::new(cmd_name)
            .args(&args.command[1..])
            .spawn()
            .expect("Command failed");

        child.wait().expect("Child exit code");
        println!();
        std::thread::sleep(Duration::new(args.duration, 0));
    }
}
