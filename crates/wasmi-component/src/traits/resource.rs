use as_any::AsAny;
use std::fmt::Debug;

pub trait Resource: Debug + AsAny {}
