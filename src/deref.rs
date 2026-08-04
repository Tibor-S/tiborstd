#[macro_export]
macro_rules! impl_deref {
    ($st:ident$(::$ex:ident)*$(<$($gty:ident),+>)?, $target:ident$(::$target_ex:ident)*$(<$($target_gty:ty),+>)?, $($field:tt)+) => {


        impl$(<$($gty),+>)? std::ops::Deref for $st$(::$ex)*$(<$($gty),+>)? {
            type Target = $target$(::$target_ex)*$(<$($target_gty),+>)?;

            fn deref(&self) -> &Self::Target {
                &self.$($field)+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_mut {
    ($st:ident$(::$ex:ident)*$(<$($gty:ident),+>)?, $target:ident$(::$target_ex:ident)*$(<$($target_gty:ty),+>)?, $($field:tt)+) => {
        $crate::impl_deref!($st$(::$ex)*$(<$($gty),+>)?, $target$(::$target_ex)*$(<$($target_gty),+>)?, $($field)+);

        impl$(<$($gty),+>)? std::ops::DerefMut for $st$(::$ex)*$(<$($gty),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$($field)+
            }
        }
    };
}
