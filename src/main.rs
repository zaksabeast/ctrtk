use std::io;
use std::str;
use structopt::StructOpt;

mod cro;
mod utils;

/// A tool to help examine and learn about 3ds file formats
#[derive(StructOpt)]
#[structopt(name = "ctr-cro-cli")]
struct Opt {
    #[structopt(subcommand)]
    subcommands: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Handle cro files
    Cro {
        /// cro file path
        cro_path: String,
    },
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    match opt.subcommands {
        SubCommand::Cro { cro_path } => {
            let raw_cro = utils::read_to_vec(&cro_path)?;
            let cro = cro::Cro::new(raw_cro);

            println!("Cro name: {}", cro.get_name());
            println!("Header: {:#?}", cro.get_header());
        }
    }

    Ok(())
}
