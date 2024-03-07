use clap::Parser;
use gg_transl::Args;

fn main() {
    let args = Args::parse();

    println!("{args:#?}");
}
