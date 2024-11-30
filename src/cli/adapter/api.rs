use std::io::Result;
use structopt::StructOpt;

use super::IAdapter;

#[derive(Debug, StructOpt)]
pub enum ApiAdapter {
    Init {
        #[structopt()]
        name: String,
    },
    Module {
        #[structopt()]
        name: String,
    },
}

pub fn init_api(name: &String) -> Result<()> {
    todo!("Implement Todo App");
}

pub fn module(name: &String) -> Result<()> {
    todo!("Implement Todo App");
}

impl IAdapter<ApiAdapter> for ApiAdapter {
    fn matcher(self) -> Result<()> {
        match self {
            Self::Init { name } => init_api(&name),
            Self::Module { name } => module(&name),
        }?;
        Ok(())
    }
}
