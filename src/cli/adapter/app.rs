use std::io::Result;
use structopt::StructOpt;

use super::IAdapter;

#[derive(Debug, StructOpt)]
pub enum AppAdapter {
    Init {
        #[structopt()]
        name: String,
    },
    Component {
        #[structopt()]
        name: String,
    },
    Screen {
        #[structopt()]
        name: String,
    },
}

pub fn init_app(name: String) -> Result<()> {
    todo!("Implement Todo App INIT: {}", name);
}

pub fn component(name: String) -> Result<()> {
    todo!("Implement Todo App COMPONENT: {}", name);
}

pub fn screen(name: String) -> Result<()> {
    todo!("Implement Todo App SCREEN: {}", name);
}

impl IAdapter<AppAdapter> for AppAdapter {
    fn matcher(self) -> Result<()> {
        match self {
            Self::Init { name } => init_app(name),
            Self::Component { name } => component(name),
            Self::Screen { name } => screen(name),
        }?;
        Ok(())
    }
}
