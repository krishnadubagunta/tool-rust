use std::io::Result;

use structopt::StructOpt;
mod api;
mod app;

trait IAdapter<T> {
    fn matcher(self) -> Result<()>;
}

#[derive(Debug, StructOpt)]
pub enum Apps {
    App {
        #[structopt(subcommand)]
        app: app::AppAdapter,
    },
    Api {
        #[structopt(subcommand)]
        app: api::ApiAdapter,
    },
}

impl Apps {
    pub fn match_apps(self) -> Result<()> {
        match self {
            Apps::App { app } => app.matcher(),
            Apps::Api { app } => app.matcher(),
        }?;

        Ok(())
    }
}
