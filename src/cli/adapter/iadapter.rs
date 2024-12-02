use std::io::Result;

pub trait IAdapter<T> {
    fn matcher(self) -> Result<()>;
}
