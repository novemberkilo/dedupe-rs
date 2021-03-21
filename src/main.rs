use dedupe::app::lookup_by_size;
use dedupe::app::lookup_by_hash;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    store: String,
}

fn main() {
    let args = Cli::from_args();
    let grouped_by_size = lookup_by_size(args.store).unwrap_or(vec![]);
    let grouped_by_hash = lookup_by_hash(&grouped_by_size).unwrap_or(vec![]);
    for dupes in grouped_by_hash {
        for f in dupes {
            println!("{}",f.path.to_string_lossy());
        }
        println!("{}","----");
    }
}
