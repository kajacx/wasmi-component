use crate::{ComponentValue, ConvertResult, Lift};

impl Lift<Self> for () {
    fn lift_owned(&self) -> ConvertResult<Self> {
        Ok(())
    }

    fn lift_to(&self, _target: &mut ()) -> ConvertResult<()> {
        Ok(())
    }
}

impl<'a, T: ComponentValue> Lift<(T,)> for (T::Borrowed<'a>,) {
    fn lift_owned(&self) -> ConvertResult<(T,)> {
        Ok((self.0.lift_owned()?,))
    }

    fn lift_to(&self, target: &mut (T,)) -> ConvertResult<()> {
        self.0.lift_to(&mut target.0)
    }
}

impl<'a, T0: ComponentValue, T1: ComponentValue> Lift<(T0, T1)>
    for (T0::Borrowed<'a>, T1::Borrowed<'a>)
{
    fn lift_owned(&self) -> ConvertResult<(T0, T1)> {
        Ok((self.0.lift_owned()?, self.1.lift_owned()?))
    }

    fn lift_to(&self, target: &mut (T0, T1)) -> ConvertResult<()> {
        self.0.lift_to(&mut target.0)?;
        self.1.lift_to(&mut target.1)?;
        Ok(())
    }
}

impl<'a, T0: ComponentValue, T1: ComponentValue, T2: ComponentValue> Lift<(T0, T1, T2)>
    for (T0::Borrowed<'a>, T1::Borrowed<'a>, T2::Borrowed<'a>)
{
    fn lift_owned(&self) -> ConvertResult<(T0, T1, T2)> {
        Ok((
            self.0.lift_owned()?,
            self.1.lift_owned()?,
            self.2.lift_owned()?,
        ))
    }

    fn lift_to(&self, target: &mut (T0, T1, T2)) -> ConvertResult<()> {
        self.0.lift_to(&mut target.0)?;
        self.1.lift_to(&mut target.1)?;
        self.2.lift_to(&mut target.2)?;
        Ok(())
    }
}
