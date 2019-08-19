use crate::boolean::{BoolIfElse, IfElse};
use crate::expr::deriv::Deriv;
use crate::expr::{Expr, Expression, One, Reduce, Zero};
use crate::ident::TypeEq;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub struct Var<T, Tag> {
    value: T,
    _tag: PhantomData<*const Tag>,
}

impl<T: Debug, Tag: Default + Debug> Debug for Var<T, Tag> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}({:?})", Tag::default(), self.value)
    }
}

impl<T: Clone, Tag> Clone for Var<T, Tag> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _tag: PhantomData,
        }
    }
}

impl<T: Copy, Tag> Copy for Var<T, Tag> {}

impl<T: Default, Tag> Default for Var<T, Tag> {
    #[inline]
    fn default() -> Self {
        Self {
            value: T::default(),
            _tag: PhantomData,
        }
    }
}

impl<T, Tag> Var<T, Tag> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            _tag: PhantomData,
        }
    }
}

impl<T, Tag> Expression for Var<T, Tag> {
    type Output = T;

    #[inline]
    fn eval(self) -> Self::Output {
        self.value
    }
}

impl<T, TagA, TagB, Same> Deriv<TagB> for Var<T, TagA>
where
    TagA: TypeEq<TagB, Output = Same>,
    Same: BoolIfElse<One, Zero>,
    IfElse<Same, One, Zero>: Default,
{
    type Output = IfElse<Same, One, Zero>;

    #[inline]
    fn deriv(&self) -> Self::Output {
        IfElse::<Same, One, Zero>::default()
    }
}

impl<T, Tag> Reduce for Var<T, Tag> {
    type Output = Self;
    #[inline(always)]
    fn reduce(self) -> Self::Output {
        self
    }
}

#[inline]
pub fn var<T, Tag>(x: T, _: Tag) -> Expr<Var<T, Tag>> {
    Expr(Var::new(x))
}
