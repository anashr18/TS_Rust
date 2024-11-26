# TS_Rust_C++
## TS vars
Modifier	Access Level
public	Accessible anywhere
private	Accessible only within the defining class
protected	Accessible within the class and subclasses

### Key Differences Between Pointer and Reference Concepts

| Feature           | **Go Pointer**                   | **C++ Reference**              | **Rust Borrowing**                   |
| ----------------- | -------------------------------- | ------------------------------ | ------------------------------------ |
| **Definition**    | Memory address of a value        | Alias for an existing variable | Borrowed access to a value           |
| **Syntax**        | `*T` (pointer to `T`)            | `T&`                           | `&T` (immutable), `&mut T` (mutable) |
| **Rebinding**     | Can point to a different address | Cannot be reassigned           | Not reassignable during lifetime     |
| **Dereferencing** | Explicit (`*ptr`)                | Implicit                       | Explicit (`*borrow`)                 |
| **Nullability**   | Can be `nil`                     | Cannot be `nullptr`            | No null concept                      |
