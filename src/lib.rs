pub mod boolean;
pub mod expr;
pub mod func;
pub mod ident;
pub mod option;
pub mod paren;
pub mod tags;

mod macros;

pub mod prelude {
    pub use crate::boolean::{False, True};
    pub use crate::func::{Apply, CanApply, Func};
    pub use crate::option::{NoneType, SomeType};
    pub use crate::{paren, paren::Paren, paren_pat, Paren};
    pub use crate::{T0, T1};
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct T0;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct T1;
