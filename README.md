# Side-Rel
Relative side relation for 2d geometric orientation

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

Get reference to relative orientation value
```rust 
fn value(&self) -> &Is
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
