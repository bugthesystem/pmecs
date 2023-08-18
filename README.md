# pmecs
Poor man's Entity Component System

## Getting started

```
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
