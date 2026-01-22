macro_rules! MY_WHILE_PTR_LAST_NOT_EQUAL { ($ptr1:expr, $ptr2:expr) =>
    {
        while $ptr1[-1] != $ptr2[-1]
        {
            $ptr1.minus_minus();
            $ptr2.minus_minus();
        }
    }
}