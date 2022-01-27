use bevy::{
    math::Vec3,
    prelude::{Commands, Plugin, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    colors::{self, Colors},
    config,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Board>()
            .add_startup_system(create_board);
    }
}

pub struct Board {
    tiles: [[Colors; 8]; 8],
}

impl Board {
    pub fn tiles(&self) -> &[[Colors; 8]; 8] {
        &self.tiles
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: [
                [
                    Colors::Orange,
                    Colors::Red,
                    Colors::Green,
                    Colors::Pink,
                    Colors::Yellow,
                    Colors::Blue,
                    Colors::Purple,
                    Colors::Brown,
                ],
                [
                    Colors::Blue,
                    Colors::Orange,
                    Colors::Pink,
                    Colors::Purple,
                    Colors::Red,
                    Colors::Yellow,
                    Colors::Brown,
                    Colors::Green,
                ],
                [
                    Colors::Purple,
                    Colors::Pink,
                    Colors::Orange,
                    Colors::Blue,
                    Colors::Green,
                    Colors::Brown,
                    Colors::Yellow,
                    Colors::Red,
                ],
                [
                    Colors::Pink,
                    Colors::Green,
                    Colors::Red,
                    Colors::Orange,
                    Colors::Brown,
                    Colors::Purple,
                    Colors::Blue,
                    Colors::Yellow,
                ],
                [
                    Colors::Yellow,
                    Colors::Blue,
                    Colors::Purple,
                    Colors::Brown,
                    Colors::Orange,
                    Colors::Red,
                    Colors::Green,
                    Colors::Pink,
                ],
                [
                    Colors::Red,
                    Colors::Yellow,
                    Colors::Brown,
                    Colors::Green,
                    Colors::Blue,
                    Colors::Orange,
                    Colors::Pink,
                    Colors::Purple,
                ],
                [
                    Colors::Green,
                    Colors::Brown,
                    Colors::Yellow,
                    Colors::Red,
                    Colors::Purple,
                    Colors::Pink,
                    Colors::Orange,
                    Colors::Blue,
                ],
                [
                    Colors::Brown,
                    Colors::Purple,
                    Colors::Blue,
                    Colors::Yellow,
                    Colors::Pink,
                    Colors::Green,
                    Colors::Red,
                    Colors::Orange,
                ],
            ],
        }
    }
}

fn create_board(mut commands: Commands, board: Res<Board>) {
    let tile_size = config::BOARD_SIZE / 8.0;
    let scale = Vec3::new(tile_size, tile_size, 0.0);
    let start = -config::BOARD_SIZE / 2.0 + tile_size / 2.0;

    for x in 0..8 {
        for y in 0..8 {
            let color = match board.tiles()[x][y] {
                Colors::Orange => colors::ORANGE,
                Colors::Blue => colors::BLUE,
                Colors::Purple => colors::PURPLE,
                Colors::Pink => colors::PINK,
                Colors::Yellow => colors::YELLOW,
                Colors::Red => colors::RED,
                Colors::Green => colors::GREEN,
                Colors::Brown => colors::BROWN,
            };
            let translation = Vec3::new(
                start + (x as f32) * tile_size,
                start + (y as f32) * tile_size,
                0.0,
            );

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                transform: Transform {
                    translation,
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
