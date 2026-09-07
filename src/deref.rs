#[macro_export]
macro_rules! impl_deref {
    (
        $st:ident$(::$ex:ident)*$(
            <$($glt:lifetime),* $(,)? $($gty:ident),*>
        )?,
        $target:ident$(::$target_ex:ident)*$(
            <$($target_gty:ty),*>
        )?,
        $($field:tt)+
    ) => {


        impl$(<$($glt,)* $($gty),*>)? std::ops::Deref for $st$(::$ex)*$(<$($glt,)* $($gty),*>)? {
            type Target = $target$(::$target_ex)*$(<$($target_gty),*>)?;

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
        $target:ident$(::$target_ex:ident)*$(
            <$($target_gty:ty),*>
        )?,
        $($field:tt)+
    ) => {
        $crate::impl_deref!($st$(::$ex)*$(<$($glt),* $($gty),*>)?, $target$(::$target_ex)*$(<$($target_gty),+>)?, $($field)+);

        impl$(<$($glt,)* $($gty),*>)? std::ops::DerefMut for $st$(::$ex)*$(<$($glt,)* $($gty),*>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$($field)+
            }
        }
    };
}
