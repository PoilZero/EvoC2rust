I need to translate the given C code to Rust following the rules specified. Let me analyze the C code and translate it step by step.

The C code is:
```c
int vec_insert_(char **data, int *length, int *capacity, int memsz, int idx)
{
    int err = vec_expand_(data, length, capacity, memsz);
    if (err)
        return err;
    memmove(*data + (idx + 1) * memsz, *data + idx * memsz, (*length - idx) * memsz);
    return 0;
}