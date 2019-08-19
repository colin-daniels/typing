use crate::expr::deriv::{Deriv, Dv};

pub mod deriv;
pub mod ops;
pub mod var;

pub trait Expression {
    type Output;
    fn eval(self) -> Self::Output;
}

pub type ExprOut<T> = <T as Expression>::Output;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Default)]
pub struct Zero;

#[derive(Copy, Clone, Debug, Default)]
pub struct One;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Default, Debug, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct Expr<T>(pub T);

impl<T: Expression> Expr<T> {
    #[inline]
    pub fn eval(self) -> ExprOut<T> {
        self.0.eval()
    }
}

impl<T: Expression> Expression for Expr<T> {
    type Output = T::Output;
    #[inline]
    fn eval(self) -> Self::Output {
        self.0.eval()
    }
}

impl<T> Expr<T> {
    #[inline]
    pub fn deriv<Tag>(&self, _: Tag) -> Expr<Dv<T, Tag>>
    where
        T: Deriv<Tag>,
    {
        Expr(self.0.deriv())
    }

    #[inline]
    pub fn reduce(self) -> Expr<Reduced<T>>
    where
        T: Reduce,
    {
        Expr(self.0.reduce())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Reduce {
    type Output;
    fn reduce(self) -> Self::Output;
}

pub type Reduced<T> = <T as Reduce>::Output;

#[cfg(test)]
mod tests {
    use crate::expr::var::var;
    use crate::tags::{X, Y, Z};

    #[test]
    fn expr() {
        let x = var(8.0, X);
        let y = var(2.0, Y);
        let z = var(-1.0, Z);

        let w = (x + y * x) / z;
        assert_eq!(w.eval(), -24.0);

        let w_dy = w.deriv(Y);
        assert_eq!(w_dy.eval(), -8.0);

        let w_dz = w.deriv(Z);
        assert_eq!(w_dz.eval(), -24.0);
    }
}
