use structopt::{StructOpt, clap, clap::AppSettings};
pub mod up;

#[derive(StructOpt, Debug)]
pub enum M2Subcommands {
    Up(up::Up),
    #[structopt(external_subcommand)]
    PassThrough(Vec<String>)
}
