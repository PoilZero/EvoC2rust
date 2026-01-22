I need to translate the given C code to Rust following the rules specified. Let me analyze the C code and translate it step by step.

The C code is:
```c
void vec_splice_(char **data, int *length, int *capacity, int memsz, int start, int count)
{
    (void)capacity;
    memmove(*data + start * memsz, *data + (start + count) * memsz, (*length - start - count) * memsz);
}