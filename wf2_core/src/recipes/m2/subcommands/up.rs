use structopt::{StructOpt, clap, clap::AppSettings};
/// Bring up the containers
#[derive(StructOpt, Debug, Clone)]
#[structopt(alias = "up")]
pub struct Up {
    #[structopt(short, long)]
    attached: bool,
    #[structopt(short, long)]
    clean: bool,
    #[structopt(short, long)]
    build: bool,
    #[structopt(short, long)]
    sync: Option<Vec<std::path::PathBuf>>,
}
