extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const WINDOW_SIZE: [u32; 2] = [800, 600];
const PLAYER_RADIUS: f64 = 20.0;
const ENEMY_RADIUS: f64 = 15.0;
const NUM_ENEMIES: usize = 5;
const NUM_OBSTACLES: usize = 10;
const OBSTACLE_SIZE: f64 = 30.0;
const BULLET_RADIUS: f64 = 5.0;

struct Player {
    x: f64,
    y: f64,
    velocity: [f64; 2],
    shooting_direction: Option<[f64; 2]>,
    last_direction: [f64; 2],
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
            shooting_direction: None,
            last_direction: [0.0, 0.0],
        }
    }

    fn update(&mut self) {
        self.x += self.velocity[0];
        self.y += self.velocity[1];
        if self.velocity != [0.0, 0.0] {
            self.last_direction = self.velocity;
        }
    }

    fn shoot(&mut self, direction: [f64; 2]) -> Bullet {
        Bullet {
            x: self.x,
            y: self.y,
            velocity: direction,
        }
    }
}

impl Bullet {
    fn update(&mut self) {
        self.x += self.velocity[0];
        self.y += self.velocity[1];
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
    let mut window: PistonWindow = WindowSettings::new("Homing Enemies", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut player = Player::new();
    let mut enemies: Vec<Enemy> = (0..NUM_ENEMIES).map(|_| Enemy::new()).collect();
    let mut obstacles: Vec<Obstacle> = (0..NUM_OBSTACLES).map(|_| Obstacle::new()).collect();
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
            }

            // Update bullets
            let mut to_remove = Vec::new();
            for i in 0..bullets.len() {
                bullets[i].update();
                if bullets[i].is_outside_window() {
                    to_remove.push(i);
                }
            }
            for &index in to_remove.iter().rev() {
                bullets.remove(index);
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => player.velocity[1] = -5.0,
                Key::S => player.velocity[1] = 5.0,
                Key::A => player.velocity[0] = -5.0,
                Key::D => player.velocity[0] = 5.0,
                Key::Space => {
                    // Shoot in the current direction
                    if player.velocity != [0.0, 0.0] {
                        player.shooting_direction = Some(player.velocity);
                        let bullet = player.shoot(player.velocity);
                        bullets.push(bullet);
                    } else {
                        // Shoot in the last direction
                        player.shooting_direction = Some(player.last_direction);
                        let bullet = player.shoot(player.last_direction);
                        bullets.push(bullet);
                    }
                }
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W | Key::S => player.velocity[1] = 0.0,
                Key::A | Key::D => player.velocity[0] = 0.0,
                Key::Space => {
                    // Stop shooting
                    player.shooting_direction = None;
                }
                _ => {}
            }
        }
    }
}
