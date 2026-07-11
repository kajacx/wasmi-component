use std::fmt::Write;

use crate::{ConvertResult, View};

impl<'a> View<String> for &'a str {
    fn lift_owned(&self) -> ConvertResult<String> {
        Ok(self.to_string())
    }

    fn lift_to(&self, target: &mut String) -> ConvertResult<()> {
        target.clear();
        write!(target, "{self}").unwrap();
        Ok(())
    }
}
