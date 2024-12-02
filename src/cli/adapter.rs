mod api;
mod app;
mod iadapter;

use iadapter::IAdapter;
use std::io::Result;
use structopt::StructOpt;

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
