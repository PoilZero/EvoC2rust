Okay, let's tackle this translation problem. The user wants me to translate a specific C macro to Rust following all the given rules. 

First, I need to understand what the macro does. The source is:

```c
#define vec_pop(v) (v)->data[--(v)->length]