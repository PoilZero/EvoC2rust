Okay, let's tackle this Rust translation for the given C macro. The user provided several examples of how to translate C macros to Rust, so I need to follow those patterns carefully.

First, I'll look at the source macro:

```c
#define vec_swapsplice(v, start, count) (vec_swapsplice_(vec_unpack_(v), start, count), (v)->length -= (count))