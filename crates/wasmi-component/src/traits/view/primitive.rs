use anyhow::Result;

use crate::View;

impl View<Self> for i32 {
    fn lift_owned(&self) -> Result<Self> {
        Ok(*self)
    }

    fn lift_to(&self, target: &mut Self) -> Result<()> {
        *target = *self;
        Ok(())
    }
}

impl View<Self> for u8 {
    fn lift_owned(&self) -> Result<Self> {
        Ok(*self)
    }

    fn lift_to(&self, target: &mut Self) -> Result<()> {
        *target = *self;
        Ok(())
    }
}

impl View<Self> for u32 {
    fn lift_owned(&self) -> Result<Self> {
        Ok(*self)
    }

    fn lift_to(&self, target: &mut Self) -> Result<()> {
        *target = *self;
        Ok(())
    }
}

impl View<Self> for u64 {
    fn lift_owned(&self) -> Result<Self> {
        Ok(*self)
    }

    fn lift_to(&self, target: &mut Self) -> Result<()> {
        *target = *self;
        Ok(())
    }
}

impl View<Self> for f32 {
    fn lift_owned(&self) -> Result<Self> {
        Ok(*self)
    }

    fn lift_to(&self, target: &mut Self) -> Result<()> {
        *target = *self;
        Ok(())
    }
}
