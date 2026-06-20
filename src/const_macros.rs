
#[macro_export]
macro_rules! c_for {
    ($it:ident in $low:literal..$high:literal $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:literal..$high:ident $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..$high:literal $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..$high:ident $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };

    ($it:ident in $low:literal..=$high:literal $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:literal..=$high:ident $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..=$high:literal $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..=$high:ident $exec:stmt) => {
        $crate::c_for_raw!($low; $high; $it; $exec)
    };
}

#[macro_export]
macro_rules! c_for_rev {
    ($it:ident in $low:literal..$high:literal $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:literal..$high:ident $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..$high:literal $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..$high:ident $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };

    ($it:ident in $low:literal..=$high:literal $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:literal..=$high:ident $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..=$high:literal $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };
    ($it:ident in $low:ident..=$high:ident $exec:stmt) => {
        $crate::c_for_rev_raw!($low; $high; $it; $exec)
    };
}

#[macro_export]
macro_rules! c_for_raw {
    ($low:expr; $high:expr; $it:ident; $exec:stmt) => {
        {
            let mut $it = $low;
            while $it < $high {
                $exec
                $it += 1;
            }
        }
    };
    ($low:expr; =$high:expr; $it:ident; $exec:stmt) => {
        {
            let mut $it = $low;
            while $it <= $high {
                $exec
                $it += 1;
            }
        }
    };
}

#[macro_export]
macro_rules! c_for_rev_raw {
    ($low:expr; $high:expr; $it:ident; $exec:stmt) => {
        {
            let mut $it = $high;
            while $it > $low {
                $it -= 1;
                $exec
            }
        }
    };
    ($low:expr; =$high:expr; $it:ident; $exec:stmt) => {
        {
            let mut $it = $high;
            while $it >= $low {
                $exec
                $it -= 1;
            }
        }
    };
}
