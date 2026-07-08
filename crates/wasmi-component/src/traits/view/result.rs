use anyhow::Result;

use crate::{CompValue, View};

impl<'a, T: CompValue, E: CompValue> View<Result<T, E>>
    for Result<T::Borrowed<'a>, E::Borrowed<'a>>
{
    fn lift_owned(&self) -> Result<Result<T, E>> {
        let owned = match self {
            Ok(ok) => Ok(ok.lift_owned()?),
            Err(err) => Err(err.lift_owned()?),
        };

        Ok(owned)
    }

    fn lift_to(&self, target: &mut Result<T, E>) -> Result<()> {
        match self {
            Ok(self_ok) => {
                if let Ok(target_ok) = target {
                    self_ok.lift_to(target_ok)
                } else {
                    *target = Ok(self_ok.lift_owned()?);
                    Ok(())
                }
            }
            Err(self_err) => {
                if let Err(target_err) = target {
                    self_err.lift_to(target_err)
                } else {
                    *target = Err(self_err.lift_owned()?);
                    Ok(())
                }
            }
        }
    }
}

impl<'a, T: CompValue> View<Option<T>> for Option<T::Borrowed<'a>> {
    fn lift_owned(&self) -> Result<Option<T>> {
        let owned = match self {
            None => None,
            Some(val) => Some(val.lift_owned()?),
        };

        Ok(owned)
    }

    fn lift_to(&self, target: &mut Option<T>) -> Result<()> {
        match self {
            None => {
                *target = None;
                Ok(())
            }
            Self::Some(self_val) => {
                if let Some(target_val) = target {
                    self_val.lift_to(target_val)
                } else {
                    *target = Some(self_val.lift_owned()?);
                    Ok(())
                }
            }
        }
    }
}
