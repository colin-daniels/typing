use crate::boolean::IfElse;
use crate::impl_func;
use std::marker::PhantomData;

pub trait CanApply<F> {
    type Output;
    fn apply(self) -> Self::Output;
}

pub trait Func<I> {
    type Output;
    fn call(input: I) -> Self::Output;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl<F, I> CanApply<F> for I
where
    F: Func<I>,
{
    type Output = <F as Func<I>>::Output;

    #[inline]
    fn apply(self) -> Self::Output {
        F::call(self)
    }
}

pub type Apply<F, I> = <I as CanApply<F>>::Output;
pub type ApplyIfElse<B, TrueFn, FalseFn, I> = Apply<IfElse<B, TrueFn, FalseFn>, I>;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Chain<F>(PhantomData<*const F>);

impl<F> Clone for Chain<F> {
    #[inline]
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<F> Copy for Chain<F> {}

pub fn chain<F>(_: F) -> Chain<F> {
    Chain(PhantomData)
}

impl<F, Fs, I> Func<I> for Chain<(F, Fs)>
where
    I: CanApply<F>,
    Apply<F, I>: CanApply<Chain<Fs>>,
{
    type Output = Apply<Chain<Fs>, Apply<F, I>>;

    #[inline]
    fn call(input: I) -> Self::Output {
        input.apply().apply()
    }
}

impl<I> Func<I> for Chain<()> {
    type Output = I;

    #[inline(always)]
    fn call(input: I) -> Self::Output {
        input
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct Identity;

impl_func!(for Identity {
    #[inline]
    [T] |x: T| -> T { x }
});

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_std_binop_fn {
    ($name:ident, $trait:ident, $method:ident) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name;
        impl<L, R> Func<(L, R)> for $name
        where
            L: std::ops::$trait<R>,
        {
            type Output = <L as std::ops::$trait<R>>::Output;
            #[inline]
            fn call((l, r): (L, R)) -> Self::Output {
                std::ops::$trait::$method(l, r)
            }
        }
    };
}
impl_std_binop_fn!(AddFn, Add, add);
impl_std_binop_fn!(BitAndFn, BitAnd, bitand);
impl_std_binop_fn!(BitOrFn, BitOr, bitor);
impl_std_binop_fn!(BitXorFn, BitXor, bitxor);
impl_std_binop_fn!(DivFn, Div, div);
impl_std_binop_fn!(MulFn, Mul, mul);
impl_std_binop_fn!(RemFn, Rem, rem);
impl_std_binop_fn!(ShlFn, Shl, shl);
impl_std_binop_fn!(ShrFn, Shr, shr);
impl_std_binop_fn!(SubFn, Sub, sub);
