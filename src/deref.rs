#[macro_export]
macro_rules! impl_deref {
    (
        $st:ident$(::$ex:ident)*$(
            <$($glt:lifetime),* $(,)? $($gty:ident),*>
        )?,
        $target:ty,
        $($field:tt)+
    ) => {


        impl$(<$($glt,)* $($gty),*>)? std::ops::Deref for $st$(::$ex)*$(<$($glt,)* $($gty),*>)? {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$($field)+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_mut {
    (
        $st:ident$(::$ex:ident)*$(
            <$($glt:lifetime),* $(,)? $($gty:ident),*>
        )?,
        $target:ty,
        $($field:tt)+
    ) => {
        $crate::impl_deref!($st$(::$ex)*$(<$($glt),* $($gty),*>)?, $target, $($field)+);

        impl$(<$($glt,)* $($gty),*>)? std::ops::DerefMut for $st$(::$ex)*$(<$($glt,)* $($gty),*>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$($field)+
            }
        }
    };
}
