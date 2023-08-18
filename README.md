# pmecs
Poor man's Entity Component System

## ECS 101
- **Entities**: A simple identifier for game objects.
- **Components**: Data containers associated with entities.
- **Systems**: Logic units that operate on entities with certain component combinations.
- **World**: Contains all entities and components.
- **Events**: Messages or data that systems can emit or listen for.


## Heads up
⚠️ Please consider using established ECS libraries like the following as they offer optimized storage, efficient querying mechanisms, and a host of other features out of the box.
- [`specs`](https://crates.io/crates/specs)
- [`legion`](https://crates.io/crates/legion)
- [`hecs`](https://crates.io/crates/hecs)
- [`bevy-ecs`](https://crates.io/crates/bevy_ecs)

## Getting started

```rust
use pmecs::{Entity, World};

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Velocity {
    dx: i32,
    dy: i32,
}

struct Paddle {
    height: i32,
}

fn main() {
    let mut world = World::new();

    let ball = world.create_entity();
    world.add_component(ball, Position { x: 10, y: 10 });
    world.add_component(ball, Velocity { dx: 1, dy: 1 });

    let left_paddle = world.create_entity();
    world.add_component(left_paddle, Position { x: 0, y: 5 });
    world.add_component(left_paddle, Paddle { height: 5 });

    let right_paddle = world.create_entity();
    world.add_component(right_paddle, Position { x: 20, y: 5 });
    world.add_component(right_paddle, Paddle { height: 5 });

    loop {
        move_ball_system(&mut world, ball, left_paddle, right_paddle);
        render_system(&world, ball, left_paddle, right_paddle);
        sleep(Duration::from_millis(100));
    }
}

```

## License

[UNLICENSE](https://github.com/ziyasal/pmecs/blob/main/LICENSE)

> _This crate is developed to be part of Λ.R.Ξ.N.Λ 2D game engine._
