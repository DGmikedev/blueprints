use std::mem;
pub mod mc_format {
    #[warn(unused_macros)]
    #[macro_export] macro_rules! pr_ls {
        ($x:expr) => {
            println!("{:?}  ------------------\n", stringify!($x));
            for (i, value) in $x.clone().into_iter().enumerate() {
                println!("{i} - {:?}", value)
            }
            println!("\n--------------- \n");
        };
    }

    #[warn(unused_macros)]
    #[macro_export] macro_rules! pr_sep {
        () => {
            println!("-----------------\n")
        };
        ($x:expr) => {
            println!("{}  --------------\n", $x)
        };
    }

    #[warn(unused_macros)]
    #[macro_export] macro_rules! pr_sval {
        ($x:expr) => {
            println!("{} --> {} \n", stringify!($x), $x)
        };
    }

    #[warn(unused_macros)]
    #[macro_export] macro_rules! pr_obj {
        ($x:expr) => {
            println!("{:?} --> {:?}\n", stringify!($x), $x)
        };
    }

    #[warn(unused_macros)]
    #[macro_export] macro_rules! pr_avsb {
        ($x:expr, $y:expr) => {
            println!("{} | {:>5}", stringify!($x), stringify!($y));
            println!("{} | {:>5}", $x, $y);
            println!("\n");
        };
    }
}


