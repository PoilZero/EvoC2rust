Okay, let's tackle this Rust translation for the given C macro. The user provided several examples of how to translate C macros to Rust, so I need to follow those patterns carefully.

First, looking at the source C code for `vec_reverse`:

```c
#define vec_reverse(v)                                                                                                 \
    do                                                                                                                 \
    {                                                                                                                  \
        int i__ = (v)->length / 2;                                                                                     \
        while (i__--)                                                                                                  \
        {                                                                                                              \
            vec_swap((v), i__, (v)->length - (i__ + 1));                                                               \
        }                                                                                                              \
    } while (0)