use crate::ident::TypeId;
use crate::{T0, T1};

macro_rules! as_tuple {
    (() -> ($ret:ty)) => {
        $ret
    };
    (($h:tt $($t:tt)*) -> ()) => {
        as_tuple!(($($t)*) -> ($h))
    };
    (($h:tt $($t:tt)*) -> ($ret:ty)) => {
        as_tuple!(($($t)*) -> (($h, $ret)))
    };
}

macro_rules! impl_idents {
    (@next [$($l:tt)*][T0 $($r:tt)*] $names:tt) => {
        impl_idents!([$($l)* T1 $($r)*] $names);
    };
    (@next [$($l:tt)*][T1 $($r:tt)+] $names:tt) => {
        impl_idents!(@next [$($l)* T0][$($r)+] $names);
    };
    (@next [$($l:tt)*][T1] $names:tt) => {
        impl_idents!([$($l)* T0 T1] $names);
    };
    ([$($t:tt)+] ($name:ident $($names:ident)+)) => {
        impl_idents!([$($t)+] ($name));
        impl_idents!(@next [][$($t)+] ($($names)+));
    };
    ([$($t:tt)+] ($name:ident)) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name;
        impl TypeId for $name {
            type Id = as_tuple!(($($t)+) -> ());
        }
    };
}

impl_idents!([T0] (A B C D E F G H I J K L M N O P));
impl_idents!([T0 T0 T0 T0 T1] (Q R S T U V W X Y Z));
