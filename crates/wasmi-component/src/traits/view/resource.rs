use anyhow::Result;

use crate::{Borrow, Own, Resource, View};

impl<T: Resource> View<Self> for Borrow<T> {
    fn lift_owned(&self) -> Result<Self> {
        Ok(*self)
    }

    fn lift_to(&self, target: &mut Self) -> Result<()> {
        *target = *self;
        Ok(())
    }
}

impl<T: Resource> View<Self> for Own<T> {
    fn lift_owned(&self) -> Result<Self> {
        Ok(*self)
    }

    fn lift_to(&self, target: &mut Self) -> Result<()> {
        *target = *self;
        Ok(())
    }
}
