use bevy::{
    math::Vec3,
    prelude::{BuildChildren, Bundle, Color, Commands, Component, GlobalTransform, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{board::Colors, config};

#[derive(Component)]
pub struct WhitePiece {
    color: Colors,
    x: usize,
    y: usize,
}

impl WhitePiece {
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
        self.x
    }

    pub fn move_piece(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Component)]
pub struct BlackPiece {
    color: Colors,
    x: usize,
    y: usize,
}

impl BlackPiece {
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
        self.x
    }

    pub fn move_piece(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Bundle)]
pub struct PieceBundle<T: Component> {
    piece: T,
    transform: Transform,
    global_transform: GlobalTransform,
}

pub fn create_pieces(mut commands: Commands) {
    let tile_size = config::BOARD_SIZE / 8.0;
    let piece_size = tile_size / 1.5;

    let start = -config::BOARD_SIZE / 2.0 + tile_size / 2.0;

    //White
    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Orange, 0, 0),
            transform: Transform {
                translation: Vec3::new(start, start, 0.0),
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
                    color: config::ORANGE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Blue, 1, 0),
            transform: Transform {
                translation: Vec3::new(start + tile_size, start, 0.0),
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
                    color: config::BLUE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Purple, 2, 0),
            transform: Transform {
                translation: Vec3::new(start + 2.0 * tile_size, start, 0.0),
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
                    color: config::PURPLE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Pink, 3, 0),
            transform: Transform {
                translation: Vec3::new(start + 3.0 * tile_size, start, 0.0),
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
                    color: config::PINK,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Yellow, 4, 0),
            transform: Transform {
                translation: Vec3::new(start + 4.0 * tile_size, start, 0.0),
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
                    color: config::YELLOW,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Red, 5, 0),
            transform: Transform {
                translation: Vec3::new(start + 5.0 * tile_size, start, 0.0),
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
                    color: config::RED,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Green, 6, 0),
            transform: Transform {
                translation: Vec3::new(start + 6.0 * tile_size, start, 0.0),
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
                    color: config::GREEN,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: WhitePiece::new(Colors::Brown, 7, 0),
            transform: Transform {
                translation: Vec3::new(start + 7.0 * tile_size, start, 0.0),
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
                    color: config::BROWN,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    //Black
    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Brown, 0, 7),
            transform: Transform {
                translation: Vec3::new(start, start + 7.0 * tile_size, 0.0),
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
                    color: config::BROWN,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Green, 1, 7),
            transform: Transform {
                translation: Vec3::new(start + tile_size, start + 7.0 * tile_size, 0.0),
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
                    color: config::GREEN,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Red, 2, 7),
            transform: Transform {
                translation: Vec3::new(start + 2.0 * tile_size, start + 7.0 * tile_size, 0.0),
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
                    color: config::RED,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Yellow, 3, 7),
            transform: Transform {
                translation: Vec3::new(start + 3.0 * tile_size, start + 7.0 * tile_size, 0.0),
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
                    color: config::YELLOW,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Pink, 4, 7),
            transform: Transform {
                translation: Vec3::new(start + 4.0 * tile_size, start + 7.0 * tile_size, 0.0),
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
                    color: config::PINK,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Purple, 5, 7),
            transform: Transform {
                translation: Vec3::new(start + 5.0 * tile_size, start + 7.0 * tile_size, 0.0),
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
                    color: config::PURPLE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Blue, 6, 7),
            transform: Transform {
                translation: Vec3::new(start + 6.0 * tile_size, start + 7.0 * tile_size, 0.0),
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
                    color: config::BLUE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(PieceBundle {
            piece: BlackPiece::new(Colors::Orange, 7, 7),
            transform: Transform {
                translation: Vec3::new(start + 7.0 * tile_size, start + 7.0 * tile_size, 0.0),
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
                    color: config::ORANGE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
