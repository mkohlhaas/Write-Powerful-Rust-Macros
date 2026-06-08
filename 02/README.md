Terms:
- (matcher) => (transcriber)
- `arm` of a macro
- Macro Variable (e.g. $x)
- [Metavariables or Fragment Specifiers](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables)
- Repetition Patterns: 
  - $(…),+
  - $(…),*
  - $(…),?
- The [`ORPHAN RULE`](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules) states that a trait implementation is only allowed if
  either the trait or at least one of the types in the implementation is defined in the current crate. 
- Macro hygiene: variables defined in the macro are not exposed to the outside code; no contamination.
- [Safe Initialization of Global Data](https://docs.rs/once_cell/latest/once_cell/#safe-initialization-of-global-data)
