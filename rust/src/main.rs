use clap::Parser;
use rust::{cli::CliArgs, errors::FileError, instance::Instance, solution::Solution};

fn main() -> miette::Result<()> {
    let args = CliArgs::parse();

    if args.file.exists() {
        let instance = Instance::from(args.file);

        if args.display_instance {
            println!("{:?}", &instance);
        }

        let solution = Solution::from(&instance);

        if args.display_solution {
            println!("{:?}", solution);
        }

        Ok(())
    } else {
        Err(FileError { src: args.file })?
    }
}
