use std::error::Error;

#[derive(Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct ErrVec<T: Error>(Vec<T>);

impl<T: Error> ErrVec<T> {
    pub fn sources(&self) -> &[&(dyn Error + 'static)] {
        todo!()
    }
}
