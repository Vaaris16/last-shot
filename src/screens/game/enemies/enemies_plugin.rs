use crate::{
    GameState,
    screens::game::{game_plugin::GameSet, game_ui::score::spawn_score_text::Score},
};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::RngExt;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_enemies,
                maintain_enemies,
                despawn_enemies,
                increment_enemy_count,
            )
                .in_set(GameSet),
        )
        .insert_resource(EnemyCount::default());
    }
}

#[derive(Clone, Copy, Default)]
enum Side {
    #[default]
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Resource)]
struct EnemyCount(i32);

impl Default for EnemyCount {
    fn default() -> Self {
        Self(3)
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct Enemy {
    pos: Vec3,
    vel: Vec3,
    spawn_side: Side,
    window_y: f32,
    window_x: f32,
    enemy_size: f32,
}

impl Side {
    pub fn rand_side() -> Side {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Side::Top,
            1 => Side::Bottom,
            2 => Side::Right,
            3 => Side::Left,
            _ => unreachable!(),
        }
    }
}

impl Enemy {
    pub fn new(window: &Window) -> Self {
        Self {
            vel: Vec3::ZERO,
            pos: Vec3::ZERO,
            spawn_side: Side::rand_side(),
            window_x: window.width(),
            window_y: window.height(),
            enemy_size: Enemy::random_size(),
        }
    }

    fn random_size() -> f32 {
        let mut rng = rand::rng();

        match rng.random_range(0..3) {
            0 => 30.,
            1 => 40.,
            2 => 60.,
            _ => unreachable!(),
        }
    }

    pub fn rand_pos(&self) -> Vec3 {
        let mut rng = rand::rng();

        match self.spawn_side {
            Side::Top => Vec3::new(
                rng.random_range(-self.window_x / 2.0..self.window_x / 2.0),
                self.window_y / 2.0,
                0.0,
            ),

            Side::Bottom => Vec3::new(
                rng.random_range(-self.window_x / 2.0..self.window_x / 2.0),
                -self.window_y / 2.0,
                0.0,
            ),

            Side::Right => Vec3::new(
                self.window_x / 2.0,
                rng.random_range(-self.window_y / 2.0..self.window_y / 2.0),
                0.0,
            ),

            Side::Left => Vec3::new(
                -self.window_x / 2.0,
                rng.random_range(-self.window_y / 2.0..self.window_y / 2.0),
                0.0,
            ),
        }
    }

    pub fn rand_vel_based_sides(&self) -> Vec3 {
        let mut rng = rand::rng();

        match self.spawn_side {
            Side::Top => Vec3::new(
                rng.random_range(-3.0..3.0),
                rng.random_range(-5.0..-2.0),
                0.,
            ),

            Side::Bottom => Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(2.0..5.0), 0.),
            Side::Right => Vec3::new(
                rng.random_range(-5.0..-2.0),
                rng.random_range(-3.0..3.0),
                0.,
            ),

            Side::Left => Vec3::new(rng.random_range(2.0..5.0), rng.random_range(-3.0..3.0), 0.),
        }
    }
}

fn spawn_enemy(commands: &mut Commands, window: &Window) {
    let mut enemy = Enemy::new(&window);
    enemy.vel = enemy.rand_vel_based_sides();
    enemy.pos = enemy.rand_pos();

    commands.spawn_scene(bsn! {
        Sprite {
            image: "enemy.png",
            custom_size: Vec2::splat(enemy.enemy_size)
        }
        enemy
        Transform::from_xyz(enemy.pos.x, enemy.pos.y, enemy.pos.z)
        Collider::rectangle(enemy.enemy_size, enemy.enemy_size)
        CollisionEventsEnabled
    });
}

fn move_enemies(enemies: Query<(&Enemy, &mut Transform), With<Enemy>>) {
    for (enemy, mut enemy_trans) in enemies {
        enemy_trans.translation += enemy.vel;
    }
}

fn maintain_enemies(
    mut commands: Commands,
    window: Single<&Window>,
    enemies: Query<(), With<Enemy>>,
    enemy_count: Res<EnemyCount>,
) {
    let count_enemy = enemies.count();

    if count_enemy < enemy_count.0 as usize {
        spawn_enemy(&mut commands, &window);
    }
}

fn despawn_enemies(
    mut commands: Commands,
    window: Single<&Window>,
    enemies: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (entity, transform, enemy) in &enemies {
        let position = transform.translation;
        let enemy_size = enemy.enemy_size / 2.;

        if position.x < -half_width - enemy_size
            || position.x > half_width + enemy_size
            || position.y < -half_height - enemy_size
            || position.y > half_height + enemy_size
        {
            commands.entity(entity).despawn();
        }
    }
}

fn increment_enemy_count(mut enemy_count: ResMut<EnemyCount>, score: Res<Score>) {
    if score.0 >= 90 {
        enemy_count.0 = 10;
    } else if score.0 >= 30 {
        enemy_count.0 = 7;
    } else if score.0 >= 20 {
        enemy_count.0 = 5;
    } else if score.0 >= 10 {
        enemy_count.0 = 4;
    }
}
