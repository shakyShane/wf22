use structopt::StructOpt;
/// Bring up the containers
#[derive(StructOpt, Debug, Clone)]
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
