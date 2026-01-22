I need to translate the given C code to Rust following the rules specified. Let me analyze the C code and translate it step by step.

The C code is:
```c
int vec_reserve_(char **data, int *length, int *capacity, int memsz, int n)
{
    (void)length;
    if (n > *capacity)
    {
        void *ptr = realloc(*data, n * memsz);
        if (ptr == NULL)
            return -1;
        *data = ptr;
        *capacity = n;
    }
    return 0;
}