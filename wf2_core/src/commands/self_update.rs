use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct SelfUpdate {
    #[structopt(long, short)]
    force: bool
}
