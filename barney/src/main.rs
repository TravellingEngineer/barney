use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliEntry {}

fn main() {
    let _ = CliEntry::parse();
}
