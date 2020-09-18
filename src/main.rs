use clap::Clap;
use rand::seq::SliceRandom;

#[derive(Debug, Clap)]
struct Opts {
    /// Prints more verbose information used for debugging.
    #[clap(short, long)]
    debug: bool,
}

#[derive(Debug, Clap)]
#[clap(version = "0.1.2", name = "cargo-tips", bin_name = "cargo")]
enum Cargo {
    #[clap(name = "tips")]
    Tips(Opts),
}

/// List of tips available in this program.
/// Possible idea for future: categories of tips?
const TIPS: &'static [&'static str] = &[
    "I like you just the way you are",
    "You look beautiful today",
];

fn main() {
    let Cargo::Tips(opts) = Cargo::parse();
    if opts.debug {
        println!("{} tips available", TIPS.len());
    }

    let mut rng = rand::thread_rng();
    match TIPS.choose(&mut rng) {
        Some(tip) => {
            println!("TIP: {}", tip);
        }
        None => panic!("Could not find a tip to give you, sorry!"),
    }
}
