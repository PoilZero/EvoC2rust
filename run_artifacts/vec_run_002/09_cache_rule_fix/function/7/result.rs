I need to translate the given C code to Rust following the rules specified. Let me analyze the C code and translate it step by step.

The C code is:
```c
void vec_swap_(char **data, int *length, int *capacity, int memsz, int idx1, int idx2)
{
    unsigned char *a, *b, tmp;
    int count;
    (void)length;
    (void)capacity;
    if (idx1 == idx2)
        return;
    a = (unsigned char *)*data + idx1 * memsz;
    b = (unsigned char *)*data + idx2 * memsz;
    count = memsz;
    while (count--)
    {
        tmp = *a;
        *a = *b;
        *b = tmp;
        a++, b++;
    }
}