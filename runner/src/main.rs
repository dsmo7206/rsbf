use rsbf::run_simple;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsbf")]
struct Opt {
    #[structopt(short, long)]
    code: String,
}

fn main() {
    let code = match std::fs::read(Opt::from_args().code) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    let start = std::time::Instant::now();
    let result = run_simple(&code);
    let elapsed = start.elapsed();

    match result {
        Ok(num_insts) => {
            println!(
                "Program completed successfully in {:?} ({} instructions)",
                elapsed, num_insts
            );
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
