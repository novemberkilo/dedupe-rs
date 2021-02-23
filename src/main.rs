use structopt::StructOpt;
use dedupe::app::lookup_by_size;

#[derive(StructOpt, Debug)]
struct Cli {
    store: String
}

fn main() {
    let args = Cli::from_args();
    let _grouped_by_size = lookup_by_size(args.store);
    // let _grouped_by_hash = lookup_by_hash(&_grouped_by_size);
    // present_dupes(&_grouped_by_hash);
    println!("foo");
}
