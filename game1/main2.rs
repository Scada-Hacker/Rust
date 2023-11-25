extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use std::time::{Duration, Instant};

const WINDOW_SIZE: [u32; 2] = [800, 600];
const PLAYER_RADIUS: f64 = 20.0;
const ENEMY_RADIUS: f64 = 15.0;
const NUM_ENEMIES: usize = 5;
const NUM_OBSTACLES: usize = 10;
const OBSTACLE_SIZE: f64 = 30.0;
const BULLET_RADIUS: f64 = 5.0;
const BULLET_SPEED: f64 = 5.0;
const MAX_BULLETS: usize = 4;
const BULLET_DELAY: Duration = Duration::from_millis(500); // 500 milliseconds delay between bullets

struct Player {
    x: f64,
    y: f64,
    velocity: [f64; 2],
    shooting_direction: [f64; 2],
    last_shot_time: Option<Instant>,
    is_shooting: bool,
    active_bullets: usize,
}

struct Bullet {
    x: f64,
    y: f64,
    velocity: [f64; 2],
}

struct Enemy {
    x: f64,
    y: f64,
    speed: f64,
}

struct Obstacle {
    x: f64,
    y: f64,
}

impl Player {
    fn new() -> Self {
        Player {
            x: WINDOW_SIZE[0] as f64 / 2.0,
            y: WINDOW_SIZE[1] as f64 / 2.0,
            velocity: [0.0, 0.0],
            shooting_direction: [1.0, 0.0], // Initial shooting direction
            last_shot_time: None,
            is_shooting: false,
            active_bullets: 0,
        }
    }

    fn can_shoot(&self) -> bool {
        match self.last_shot_time {
            Some(time) => time.elapsed() >= BULLET_DELAY && self.active_bullets < MAX_BULLETS,
            None => true,
        }
    }

    fn shoot(&mut self) -> Option<Bullet> {
        if self.can_shoot() && self.is_shooting {
            self.last_shot_time = Some(Instant::now());
            self.active_bullets += 1;
            Some(Bullet {
                x: self.x,
                y: self.y,
                velocity: self.shooting_direction, // Set the bullet's velocity
            })
        } else {
            None
        }
    }

    fn update(&mut self) {
        self.x += self.velocity[0];
        self.y += self.velocity[1];
    }
}

impl Bullet {
    fn update(&mut self) {
        self.x += self.velocity[0] * BULLET_SPEED;
        self.y += self.velocity[1] * BULLET_SPEED;
    }

    fn is_outside_window(&self) -> bool {
        self.x < 0.0 || self.x > WINDOW_SIZE[0] as f64 || self.y < 0.0 || self.y > WINDOW_SIZE[1] as f64
    }
}

impl Enemy {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Enemy {
            x: rng.gen_range(0.0..WINDOW_SIZE[0] as f64),
            y: rng.gen_range(0.0..WINDOW_SIZE[1] as f64),
            speed: 0.5,
        }
    }

    fn update(&mut self, player: &Player) {
        let angle = (player.y - self.y).atan2(player.x - self.x);
        self.x += angle.cos() * self.speed;
        self.y += angle.sin() * self.speed;
    }
    fn collides_with_enemy(&self, x: f64, y: f64, radius: f64) -> bool {
        x + radius > self.x
            && x - radius < self.x + ENEMY_SIZE
            && y + radius > self.y
            && y - radius < self.y + ENEMY_SIZE
    }
}

impl Obstacle {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Obstacle {
            x: rng.gen_range(0.0..WINDOW_SIZE[0] as f64 - OBSTACLE_SIZE),
            y: rng.gen_range(0.0..WINDOW_SIZE[1] as f64 - OBSTACLE_SIZE),
        }
    }

    fn collides_with(&self, x: f64, y: f64, radius: f64) -> bool {
        x + radius > self.x
            && x - radius < self.x + OBSTACLE_SIZE
            && y + radius > self.y
            && y - radius < self.y + OBSTACLE_SIZE
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Arrow Keys to Change Bullet Direction", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut player = Player::new();
    let mut enemies: Vec<Enemy> = (0..NUM_ENEMIES).map(|_| Enemy::new()).collect();
    let obstacles: Vec<Obstacle> = (0..NUM_OBSTACLES).map(|_| Obstacle::new()).collect();
    let mut bullets: Vec<Bullet> = Vec::new();

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |c, g, _| {
                clear([1.0; 4], g);

                // Draw player
                ellipse(
                    [0.0, 0.0, 1.0, 1.0],
                    [player.x - PLAYER_RADIUS, player.y - PLAYER_RADIUS, PLAYER_RADIUS * 2.0, PLAYER_RADIUS * 2.0],
                    c.transform,
                    g,
                );

                // Draw enemies
                for enemy in &enemies {
                    ellipse(
                        [1.0, 0.0, 0.0, 1.0],
                        [enemy.x - ENEMY_RADIUS, enemy.y - ENEMY_RADIUS, ENEMY_RADIUS * 2.0, ENEMY_RADIUS * 2.0],
                        c.transform,
                        g,
                    );
                }

                // Draw obstacles
                for obstacle in &obstacles {
                    rectangle(
                        [0.8, 0.8, 0.8, 1.0],
                        [obstacle.x, obstacle.y, OBSTACLE_SIZE, OBSTACLE_SIZE],
                        c.transform,
                        g,
                    );
                }

                // Draw bullets
                for bullet in &bullets {
                    ellipse(
                        [0.0, 1.0, 0.0, 1.0],
                        [bullet.x - BULLET_RADIUS, bullet.y - BULLET_RADIUS, BULLET_RADIUS * 2.0, BULLET_RADIUS * 2.0],
                        c.transform,
                        g,
                    );
                }
            });
        }

        if let Some(update_args) = event.update_args() {
            // Update player
            player.update();

            // Check for collisions with obstacles
            for obstacle in &obstacles {
                if obstacle.collides_with(player.x, player.y, PLAYER_RADIUS) {
                    // Handle player collision with obstacle (e.g., stop movement)
                    player.velocity = [0.0, 0.0];
                }
            }

            // Update enemies
            for enemy in &mut enemies {
                enemy.update(&player);
                // check collision between enemies so they dont meld into one blob
                if enemies.collides_with_enemy(enemies.x, enemies.y, ENEMY_RADIUS) {
                    // Handle player collision with obstacle (e.g., stop movement)
                    enemy.speed = [0.0, 0.0];
                }
            }

            // Update bullets
            let mut to_remove = Vec::new();
            for i in 0..bullets.len() {
                bullets[i].update();
                if bullets[i].is_outside_window() {
                    to_remove.push(i);
                    player.active_bullets -= 1;
                }
            }
            for &index in to_remove.iter().rev() {
                bullets.remove(index);
            }

            // Shoot bullets while spacebar is held down
            if let Some(bullet) = player.shoot() {
                bullets.push(bullet);
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => player.velocity[1] = -5.0,
                Key::S => player.velocity[1] = 5.0,
                Key::A => player.velocity[0] = -5.0,
                Key::D => player.velocity[0] = 5.0,
                Key::Space => {
                    player.is_shooting = true; // Start shooting when spacebar is held down
                }
                Key::Up => player.shooting_direction = [0.0, -1.0],
                Key::Down => player.shooting_direction = [0.0, 1.0],
                Key::Left => player.shooting_direction = [-1.0, 0.0],
                Key::Right => player.shooting_direction = [1.0, 0.0],
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W | Key::S => player.velocity[1] = 0.0,
                Key::A | Key::D => player.velocity[0] = 0.0,
                Key::Space => {
                    player.is_shooting = false; // Stop shooting when spacebar is released
                }
                _ => {}
            }
        }
    }
}
