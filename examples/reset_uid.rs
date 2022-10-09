use clap::Parser;
use pe_nfc::reset_uid;

#[derive(Parser)]
pub struct Args {
    /// Path of libnfc-nci.so
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    reset_uid(&args.path).unwrap();
}
