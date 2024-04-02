# Summarize
- Ownership is a system that ensures memory is used safely by preventing undefined behaviour.
- Undefined behaviour can lead to crashes or security vulnerabilities.
- Rust checks for ownership errors at compile time to avoid these problems.
- In Rust, variables are stored on the stack and data can also be stored on the heap.
- Variables that are moved are no longer useable.

## Ownership
Ownership is primarily a discipline of heap management:
- All heap data must be owned by exactly one variable.
- Rust deallocates heap data once its owner goes out of scope.
- Ownership can be transferred by moves, which happen on assignments and function calls.
- Heap data can only be accessed through its current owner, not a previous owner.

## References and Borrowing
**Pointer Safety Principle: data should never be aliased and mutated at the same time.**
### References:
- References are non-owning pointers that refer to data on the heap.
- Used to borro data for temporary access to data without ownership.
- Prevents data from being used after it has been moved.
- References can temporarily remove RWO permissions.
### Borrowing:
- Enforced by the Rust compiler's borrow checker to prevent data races.
- Ensures only one mutable reference can access data at a time.
### Immutability vs Mutability:
- References can be mutable or immutable.
- Immutable references cannot be used to modify the data they point to.
- Mutable references can be used to modify the data they point to.
