#[macro_export]
macro_rules! impl_deref {
    ($st:ident$(<$($gty:ident),+>)?, $target:ident$(<$($target_gty:ty),+>)?, $($field:tt)+) => {


        impl$(<$($gty),+>)? std::ops::Deref for $st$(<$($gty),+>)? {
            type Target = $target$(<$($target_gty),+>)?;

            fn deref(&self) -> &Self::Target {
                &self.$($field)+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_mut {
    ($st:ident$(<$($gty:ident),+>)?, $target:ident$(<$($target_gty:ty),+>)?, $($field:tt)+) => {
        $crate::impl_deref!($st$(<$($gty),+>)?, $target$(<$($target_gty),+>)?, $($field)+);

        impl$(<$($gty),+>)? std::ops::DerefMut for $st$(<$($gty),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$($field)+
            }
        }
    };
}
