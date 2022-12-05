## Bevy ECS 


### Components: Rust structs that implement the Component trait

```rust
#[derive(Component)]
struct Position { x: f32, y: f32 }
```

### Systems: normal Rust functions

```rust
fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}
```

### Entities: a simple type containing a unique integer

```rust
struct Entity(u64);
```