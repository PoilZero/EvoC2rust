Okay, I need to translate the given C code to Rust following the specified rules. Let me first look at the C code and understand what it does.

The C function is:
```c
int vec_reserve_po2_(char **data, int *length, int *capacity, int memsz, int n)
{
    int n2 = 1;
    if (n == 0)
        return 0;
    while (n2 < n)
        n2 <<= 1;
    return vec_reserve_(data, length, capacity, memsz, n2);
}