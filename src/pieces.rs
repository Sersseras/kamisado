use bevy::{
    math::Vec3,
    prelude::{BuildChildren, Bundle, Color, Commands, Component, GlobalTransform, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{colors::Colors, config};

#[derive(Component)]
pub struct Piece {
    color: Colors,
    x: usize,
    y: usize,
}

impl Piece {
    fn new(color: Colors, x: usize, y: usize) -> Self {
        Self { color, x, y }
    }

    pub fn color(&self) -> Colors {
        self.color
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn move_piece(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Component)]
pub struct WhitePiece;

#[derive(Component)]
pub struct BlackPiece;

#[derive(Bundle)]
pub struct PieceBundle {
    transform: Transform,
    global_transform: GlobalTransform,
}

pub fn create_pieces(mut commands: Commands) {
    let tile_size = config::BOARD_SIZE / 8.0;
    let piece_size = tile_size / 1.5;

    for (i, &color) in [
        Colors::Orange,
        Colors::Blue,
        Colors::Purple,
        Colors::Pink,
        Colors::Yellow,
        Colors::Red,
        Colors::Green,
        Colors::Brown,
    ]
    .iter()
    .enumerate()
    {
        //White
        commands
            .spawn_bundle(PieceBundle {
                transform: Transform {
                    scale: Vec3::new(piece_size, piece_size, 0.0),
                    ..Default::default()
                },
                global_transform: Default::default(),
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 1.0, 1.0),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: color.color(),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        scale: Vec3::new(0.5, 0.5, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            })
            .insert(Piece::new(color, i, 0))
            .insert(WhitePiece);

        //Black
        commands
            .spawn_bundle(PieceBundle {
                transform: Transform {
                    scale: Vec3::new(piece_size, piece_size, 0.0),
                    ..Default::default()
                },
                global_transform: Default::default(),
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: color.color(),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        scale: Vec3::new(0.5, 0.5, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            })
            .insert(Piece::new(color, 7 - i, 7))
            .insert(BlackPiece);
    }
}
