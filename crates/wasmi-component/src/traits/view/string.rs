use std::fmt::Write;

use anyhow::Result;

use crate::View;

impl<'a> View<String> for &'a str {
    fn lift_owned(&self) -> Result<String> {
        Ok(self.to_string())
    }

    fn lift_to(&self, target: &mut String) -> Result<()> {
        target.clear();
        write!(target, "{self}").unwrap();
        Ok(())
    }
}
