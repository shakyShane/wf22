use structopt::StructOpt;
pub mod up;

#[derive(StructOpt, Debug)]
pub enum M2Subcommands {
    Up(up::Up),
    /// Pass through commands like `wf2 composer install -vvv`
    #[structopt(external_subcommand)]
    PassThrough(Vec<String>),
}
