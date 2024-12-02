use std::io::Result;
use structopt::StructOpt;

use super::iadapter::IAdapter;

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

pub fn init_app(_name: String) -> Result<()> {
    todo!("Implement Todo App INIT: {}", _name);
}

pub fn component(_name: String) -> Result<()> {
    todo!("Implement Todo App COMPONENT: {}", _name);
}

pub fn screen(_name: String) -> Result<()> {
    todo!("Implement Todo App SCREEN: {}", _name);
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
