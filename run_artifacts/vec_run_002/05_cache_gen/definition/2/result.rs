Okay, let's tackle this translation problem. The user wants me to translate a C code definition to Rust following specific rules. First, I need to understand all the requirements clearly.

The key points from the problem statement:
- `void*` becomes `VoidPtr` in Rust
- All `char` types become `u8`
- Arrays like `int[10]` become `Array<i32, 10>` and use `arr!` macro for initialization
- Enums become `i32` with macro_rules for each enum value (all macros uppercased)
- Pointers become `Ptr<T>`
- Global variables starting with `g_` use `Global<T>` and `global!()` macro
- Non-global variables become regular constants
- String literals use `cstr!` macro
- Structs with `typedef struct` need proper Rust struct definitions with `#[repr(C)]` and derive attributes
- For `typedef struct PreviousName { ... } PostName;`, add a type alias for `PreviousName` first

The source code given is:
```c
typedef vec_t(int) vec_int_t;