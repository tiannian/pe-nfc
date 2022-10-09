use clap::Parser;
use pe_nfc::set_uid;

#[derive(Parser)]
pub struct Args {
    /// Path of libnfc-nci.so
    #[arg(short, long)]
    path: String,

    /// nfc uid
    #[arg(short, long)]
    uid: String,
}

fn main() {
    let args = Args::parse();

    let uid = hex::decode(args.uid).unwrap();

    set_uid(&args.path, &uid).unwrap();
}
