use structopt::StructOpt;
mod adapter;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Tool",
    about = "A command line boilerplate generator written in Rust"
)]
pub struct Adapters {
    #[structopt(subcommand)]
    pub apps: adapter::Apps,
}
