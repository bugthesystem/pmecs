use std::thread::sleep;
use std::time::Duration;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    queue,
    QueueableCommand,
};
use std::io::stdout;
use pmecs::{Entity, World};


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 24;
const PADDLE_WIDTH: i32 = 1;
const PADDLE_HEIGHT: i32 = 5;

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

fn move_ball_system(world: &mut World, ball: Entity, left_paddle: Entity, right_paddle: Entity) {
    // extract position and velocity values first
    let (ball_pos, ball_vel, left_paddle_pos, right_paddle_pos) = {
        let vel = world.get_component::<Velocity>(ball).expect("Expected Velocity component").get(&ball).unwrap().clone();
        let pos = world.get_component::<Position>(ball).expect("Expected Position component").get(&ball).unwrap().clone();
        let l_paddle_pos = world.get_component::<Position>(left_paddle).expect("Expected Position component for left paddle").get(&left_paddle).unwrap().clone();
        let r_paddle_pos = world.get_component::<Position>(right_paddle).expect("Expected Position component for right paddle").get(&right_paddle).unwrap().clone();
        (pos, vel, l_paddle_pos, r_paddle_pos)
    };

    // Update ball position
    let new_pos = Position {
        x: ball_pos.x + ball_vel.dx,
        y: ball_pos.y + ball_vel.dy,
    };

    // check collision with top and bottom edges
    let mut new_vel = ball_vel;
    if new_pos.y <= 0 || new_pos.y >= SCREEN_HEIGHT - 1 {
        new_vel.dy = -new_vel.dy;
    }

    // check collision with the left paddle
    if new_pos.x == left_paddle_pos.x + PADDLE_WIDTH &&
        new_pos.y >= left_paddle_pos.y &&
        new_pos.y <= left_paddle_pos.y + PADDLE_HEIGHT {
        new_vel.dx = -new_vel.dx;
    }

    // check collision with the right paddle
    if new_pos.x == right_paddle_pos.x - PADDLE_WIDTH &&
        new_pos.y >= right_paddle_pos.y &&
        new_pos.y <= right_paddle_pos.y + PADDLE_HEIGHT {
        new_vel.dx = -new_vel.dx;
    }

    // update the ball's position and velocity in the world
    if let Some(mut pos_guard) = world.get_component_mut::<Position>(ball) {
        if let Some(pos) = pos_guard.get_mut(&ball) {
            *pos = new_pos;
        }
    }

    if let Some(mut vel_guard) = world.get_component_mut::<Velocity>(ball) {
        if let Some(vel) = vel_guard.get_mut(&ball) {
            *vel = new_vel;
        }
    }
}

fn render_system(world: &World, ball: Entity, left_paddle: Entity, right_paddle: Entity) {
    let mut stdout = stdout();

    // clear the screen
    queue!(stdout, Clear(ClearType::All)).unwrap();

    // render the ball
    if let Some(pos_guard) = world.get_component::<Position>(ball) {
        let pos_storage = &*pos_guard;
        if let Some(ball_pos) = pos_storage.get(&ball) {
            queue!(stdout, MoveTo(ball_pos.x as u16, ball_pos.y as u16)).unwrap();
            print!("O");
        }
    }

    // render the left paddle
    if let Some(pos_guard) = world.get_component::<Position>(left_paddle) {
        let pos_storage = &*pos_guard;
        if let Some(paddle_pos) = pos_storage.get(&left_paddle) {
            for y in paddle_pos.y..(paddle_pos.y + PADDLE_HEIGHT as i32) {
                stdout.queue(MoveTo(paddle_pos.x as u16, y as u16)).unwrap();
                print!("X");
            }
        }
    }

    // render the right paddle
    if let Some(pos_guard) = world.get_component::<Position>(right_paddle) {
        let pos_storage = &*pos_guard;
        if let Some(paddle_pos) = pos_storage.get(&right_paddle) {
            for y in paddle_pos.y..(paddle_pos.y + PADDLE_HEIGHT as i32) {
                stdout.queue(MoveTo(paddle_pos.x as u16, y as u16)).unwrap();
                print!("X");
            }
        }
    }

    stdout.queue(MoveTo(0, 25)).unwrap();
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

