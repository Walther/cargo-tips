use clap::Clap;
use rand::seq::SliceRandom;

#[derive(Clap)]
#[clap(version = "0.1.1")]
struct Opts {
    /// Prints more verbose information used for debugging.
    #[clap(short, long)]
    debug: bool,
}

/// List of tips available in this program.
/// Possible idea for future: categories of tips?
const TIPS: &'static [&'static str] = &[
    "I like you just the way you are",
    "You look beautiful today",
];

fn main() {
    let opts: Opts = Opts::parse();
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
