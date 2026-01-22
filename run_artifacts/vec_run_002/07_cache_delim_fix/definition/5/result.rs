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

The source code given is: `typedef vec_t(double) vec_double_t;`

Hmm, I need to figure out what `vec_t` is. From the examples, `vec_t` might be a type that's used in C for vectors. But in the context of the problem, the user hasn't provided the definition of `vec_t` here. Wait, looking back at the examples, there's a source code example that uses `vec_t`:

In the last example source code: `typedef vec_t(double) vec_double_t;`

But the problem says "You need to translate the definition only." So I have to translate this line as is.

Wait, the user provided this as the source to translate. Let me check the examples again. In the examples, they have a line like `typedef vec_t(double) vec_double_t;` and the translation is expected.

But in the problem statement, the user says: "Here are some examples" and lists several. The last example is:

Source:
```c
typedef vec_t(double) vec_double_t;