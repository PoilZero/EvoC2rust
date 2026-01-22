macro_rules! MY_VARGS_M {
    ($fmt:expr) => {
        MyVargsFunction(MYFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[]);
    };
    ($fmt:expr, $($args:expr),*) => {
        MyVargsFunction(MYFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[$(&$args), *]);
    }
}