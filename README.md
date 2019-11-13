# Side-Rel
Relative side relation for 2d geometric orientation

### coverage 
```bash 
[INFO tarpaulin] Coverage Results:
|| Uncovered Lines:
|| Tested/Total Lines:
|| src/lib.rs: 50/50
|| 
100.00% coverage, 50/50 lines covered
```

### Relative relation 
 #### enum `IS`
```rust
enum Is {
    Left,
    On,
    Right,
}
```

### methods 
New
```rust 
fn new() -> Side
```

Is left
```rust 
fn is_left(&self) -> bool
```

As left
```rust 
fn as_left(&mut self) -> &mut Self
```
Is on
```rust 
fn is_on(&self) -> bool
```
As on
```rust 
fn as_on(&mut self) -> &mut Self
```
Is right
```rust 
fn is_right(&self) -> bool
```
As right
```rust 
fn as_right(&mut self) -> &mut Self
```
Is on or left
```rust 
fn is_on_or_left(&self) -> bool
```
Is on or right
```rust 
fn is_on_or_right(&self) -> bool
```
Is on the same side as other
```rust 
fn is_same_side(&self, other: &Side) -> bool
```

### lic 
`MIT`