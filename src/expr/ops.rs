use crate::expr::deriv::{Deriv, Dv};
use crate::expr::{Expr, Expression, One, Reduce, Reduced, Zero};

macro_rules! reduce_simple {
    ($({$($c:tt $(: $c0:ident $(+ $cN:ident)*)?),*})? $typ:ty => Self) => {
        impl<$($($c $(: $c0 $(+ $cN)*)?,)*)?>  Reduce for $typ {
            type Output = Self;
            #[inline(always)]
            fn reduce(self) -> Self::Output {
                self
            }
        }
    };
    ($({$($c:tt $(: $c0:ident $(+ $cN:ident)*)?),*})? $typ:ty => $ret:tt) => {
        impl<$($($c $(: $c0 $(+ $cN)*)?,)*)?>  Reduce for $typ {
            type Output = $ret;
            #[inline(always)]
            fn reduce(self) -> Self::Output {
                $ret
            }
        }
    };
}

macro_rules! expr_unop {
    ($name:ident, $method:ident) => {
        #[derive(Copy, Clone, Default, Debug)]
        pub struct $name<T>(T);

        impl<T> ::std::ops::$name for Expr<T> {
            type Output = Expr<$name<T>>;

            #[inline]
            fn $method(self) -> Self::Output {
                Expr($name(self.0))
            }
        }

        impl<T> Expression for $name<T>
        where
            T: Expression,
            T::Output: ::std::ops::$name,
        {
            type Output = <T::Output as ::std::ops::$name>::Output;

            #[inline(always)]
            fn eval(self) -> Self::Output {
                ::std::ops::$name::$method(self.0.eval())
            }
        }
    };
}

expr_unop!(Neg, neg);
expr_unop!(Not, not);

impl<T: Deriv<Tag>, Tag> Deriv<Tag> for Neg<T> {
    type Output = Neg<Dv<T, Tag>>;

    #[inline]
    fn deriv(&self) -> Self::Output {
        Neg(self.0.deriv())
    }
}

impl<T: Reduce> Reduce for Neg<T> {
    type Output = Neg<Reduced<T>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Neg(self.0.reduce())
    }
}

reduce_simple!(Neg<Zero> => Zero);
reduce_simple!(Neg<One> => Self);

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! expr_binop {
    ($name:ident, $method:ident) => {
        #[derive(Copy, Clone, Default, Debug)]
        pub struct $name<L, R>(L, R);

        impl<L, R> ::std::ops::$name<Expr<R>> for Expr<L> {
            type Output = Expr<$name<L, R>>;

            #[inline]
            fn $method(self, rhs: Expr<R>) -> Self::Output {
                Expr($name(self.0, rhs.0))
            }
        }

        impl<L, R> Expression for $name<L, R>
        where
            L: Expression,
            R: Expression,
            L::Output: ::std::ops::$name<R::Output>,
        {
            type Output = <L::Output as ::std::ops::$name<R::Output>>::Output;

            #[inline(always)]
            fn eval(self) -> Self::Output {
                ::std::ops::$name::$method(self.0.eval(), self.1.eval())
            }
        }
    };
}

expr_binop!(Add, add);
expr_binop!(BitAnd, bitand);
expr_binop!(BitOr, bitor);
expr_binop!(BitXor, bitxor);
expr_binop!(Div, div);
expr_binop!(Mul, mul);
expr_binop!(Rem, rem);
expr_binop!(Shl, shl);
expr_binop!(Shr, shr);
expr_binop!(Sub, sub);

impl<Tag, L, R> Deriv<Tag> for Add<L, R>
where
    L: Deriv<Tag>,
    R: Deriv<Tag>,
    Add<Dv<L, Tag>, Dv<R, Tag>>: Reduce,
{
    type Output = Reduced<Add<Dv<L, Tag>, Dv<R, Tag>>>;

    #[inline]
    fn deriv(&self) -> Self::Output {
        Add(self.0.deriv(), self.1.deriv()).reduce()
    }
}

impl<Tag, L, R> Deriv<Tag> for Sub<L, R>
where
    L: Deriv<Tag>,
    R: Deriv<Tag>,
    Sub<Dv<L, Tag>, Dv<R, Tag>>: Reduce,
{
    type Output = Reduced<Sub<Dv<L, Tag>, Dv<R, Tag>>>;

    #[inline]
    fn deriv(&self) -> Self::Output {
        Sub(self.0.deriv(), self.1.deriv()).reduce()
    }
}

impl<Tag, L, R> Deriv<Tag> for Mul<L, R>
where
    L: Deriv<Tag> + Clone,
    R: Deriv<Tag> + Clone,
    Mul<Dv<L, Tag>, R>: Reduce,
    Mul<L, Dv<R, Tag>>: Reduce,
    Add<Reduced<Mul<Dv<L, Tag>, R>>, Reduced<Mul<L, Dv<R, Tag>>>>: Reduce,
{
    #[rustfmt::skip]
    type Output = Reduced<Add<
        Reduced<Mul<Dv<L, Tag>, R>>,
        Reduced<Mul<L, Dv<R, Tag>>>,
    >>;

    #[inline]
    fn deriv(&self) -> Self::Output {
        Add(
            Mul(self.0.deriv(), self.1.clone()).reduce(),
            Mul(self.0.clone(), self.1.deriv()).reduce(),
        )
        .reduce()
    }
}

impl<Tag, L, R> Deriv<Tag> for Div<L, R>
where
    L: Deriv<Tag> + Clone,
    R: Deriv<Tag> + Clone,
    Div<Dv<L, Tag>, R>: Reduce,
    Mul<Div<L, Mul<R, R>>, Dv<R, Tag>>: Reduce,
    Sub<Reduced<Div<Dv<L, Tag>, R>>, Reduced<Mul<Div<L, Mul<R, R>>, Dv<R, Tag>>>>: Reduce,
{
    #[rustfmt::skip]
    type Output = Reduced<Sub<
        Reduced<Div<Dv<L, Tag>, R>>,
        Reduced<Mul<Div<L, Mul<R, R>>, Dv<R, Tag>>>,
    >>;

    #[inline]
    fn deriv(&self) -> Self::Output {
        let Div(l, r) = self;
        Sub(
            Div(l.deriv(), r.clone()).reduce(),
            Mul(Div(l.clone(), Mul(r.clone(), r.clone())), r.deriv()).reduce(),
        )
        .reduce()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

impl<L: Reduce, R: Reduce> Reduce for Add<L, R> {
    type Output = Add<Reduced<L>, Reduced<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Add(self.0.reduce(), self.1.reduce())
    }
}

impl<L: Reduce> Reduce for Add<L, Zero> {
    type Output = Reduced<L>;

    #[inline]
    fn reduce(self) -> Self::Output {
        self.0.reduce()
    }
}

impl<R: Reduce> Reduce for Add<Zero, R> {
    type Output = Reduced<R>;

    #[inline]
    fn reduce(self) -> Self::Output {
        self.1.reduce()
    }
}

impl<L: Reduce> Reduce for Add<L, One> {
    type Output = Add<Reduced<L>, One>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Add(self.0.reduce(), One)
    }
}

impl<R: Reduce> Reduce for Add<One, R> {
    type Output = Add<One, Reduced<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Add(One, self.1.reduce())
    }
}

reduce_simple!(Add<Zero, Zero> => Zero);
reduce_simple!(Add<One, Zero> => One);
reduce_simple!(Add<Zero, One> => One);
reduce_simple!(Add<One, One> => Self);

//////////////////////////////////////////////////////////////////////////////////////////////////////

impl<L: Reduce, R: Reduce> Reduce for Sub<L, R> {
    type Output = Sub<Reduced<L>, Reduced<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Sub(self.0.reduce(), self.1.reduce())
    }
}

impl<L: Reduce> Reduce for Sub<L, Zero> {
    type Output = Reduced<L>;

    #[inline]
    fn reduce(self) -> Self::Output {
        self.0.reduce()
    }
}

impl<R: Reduce> Reduce for Sub<Zero, R>
where
    Neg<R>: Reduce,
{
    type Output = Reduced<Neg<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Neg(self.1).reduce()
    }
}

impl<L: Reduce> Reduce for Sub<L, One> {
    type Output = Sub<Reduced<L>, One>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Sub(self.0.reduce(), One)
    }
}

impl<R: Reduce> Reduce for Sub<One, R> {
    type Output = Sub<One, Reduced<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Sub(One, self.1.reduce())
    }
}

impl Reduce for Sub<Zero, One> {
    type Output = Neg<One>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Neg(One)
    }
}

reduce_simple!(Sub<Zero, Zero> => Zero);
reduce_simple!(Sub<One, Zero> => One);
// TODO: Check
reduce_simple!(Sub<One, One> => Zero);

//////////////////////////////////////////////////////////////////////////////////////////////////////

impl<L: Reduce, R: Reduce> Reduce for Mul<L, R> {
    type Output = Mul<Reduced<L>, Reduced<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Mul(self.0.reduce(), self.1.reduce())
    }
}

impl<L: Reduce> Reduce for Mul<L, One> {
    type Output = Reduced<L>;

    #[inline]
    fn reduce(self) -> Self::Output {
        self.0.reduce()
    }
}

impl<R: Reduce> Reduce for Mul<One, R> {
    type Output = Reduced<R>;

    #[inline]
    fn reduce(self) -> Self::Output {
        self.1.reduce()
    }
}

reduce_simple!({L: Reduce} Mul<L, Zero> => Zero);
reduce_simple!({R: Reduce} Mul<Zero, R> => Zero);
reduce_simple!(Mul<Zero, Zero> => Zero);
reduce_simple!(Mul<Zero, One> => Zero);
reduce_simple!(Mul<One, Zero> => Zero);
reduce_simple!(Mul<One, One> => One);

//////////////////////////////////////////////////////////////////////////////////////////////////////

impl<L: Reduce, R: Reduce> Reduce for Div<L, R> {
    type Output = Div<Reduced<L>, Reduced<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Div(self.0.reduce(), self.1.reduce())
    }
}

impl<L: Reduce> Reduce for Div<L, One> {
    type Output = Reduced<L>;

    #[inline]
    fn reduce(self) -> Self::Output {
        self.0.reduce()
    }
}

impl<R: Reduce> Reduce for Div<One, R> {
    type Output = Div<One, Reduced<R>>;

    #[inline]
    fn reduce(self) -> Self::Output {
        Div(One, self.1.reduce())
    }
}

reduce_simple!(Div<One, One> => Self);
reduce_simple!(Div<Zero, One> => Zero);
reduce_simple!({R: Reduce} Div<Zero, R> => Zero);
// Unimplemented:
//   - Div<One, Zero>
//   - Div<L, Zero>
//   - Div<Zero, Zero>

//////////////////////////////////////////////////////////////////////////////////////////////////////
