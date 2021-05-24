use rsbf::run_simple;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "rsbf")]
struct Opt {
    #[structopt(short, long)]
    code: String,
}

fn main() {
    let opt = Opt::from_args();

    let code = match std::fs::read(opt.code) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = run_simple(&code) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
