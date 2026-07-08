use anyhow::Result;

use crate::{ComponentValue, View};

impl View<Self> for () {
    fn lift_owned(&self) -> Result<Self> {
        Ok(())
    }

    fn lift_to(&self, _target: &mut ()) -> Result<()> {
        Ok(())
    }
}

impl<'a, T: ComponentValue> View<(T,)> for (T::Borrowed<'a>,) {
    fn lift_owned(&self) -> Result<(T,)> {
        Ok((self.0.lift_owned()?,))
    }

    fn lift_to(&self, target: &mut (T,)) -> Result<()> {
        self.0.lift_to(&mut target.0)
    }
}

impl<'a, T0: ComponentValue, T1: ComponentValue> View<(T0, T1)>
    for (T0::Borrowed<'a>, T1::Borrowed<'a>)
{
    fn lift_owned(&self) -> Result<(T0, T1)> {
        Ok((self.0.lift_owned()?, self.1.lift_owned()?))
    }

    fn lift_to(&self, target: &mut (T0, T1)) -> Result<()> {
        self.0.lift_to(&mut target.0)?;
        self.1.lift_to(&mut target.1)?;
        Ok(())
    }
}
