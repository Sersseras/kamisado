#![allow(clippy::needless_range_loop)]
use std::ops::{Deref, DerefMut};

use bevy::{
    input::Input,
    math::{Vec2, Vec3},
    prelude::{
        App, Color, Commands, Component, Entity, MouseButton, OrthographicCameraBundle, Query, Res,
        ResMut, Transform, With, Without,
    },
    sprite::{Sprite, SpriteBundle},
    window::Windows,
    DefaultPlugins,
};
use board::{create_board, Board};
use colors::Colors;
use pieces::{create_pieces, BlackPiece, Piece, WhitePiece};

mod board;
mod colors;
mod config;
mod pieces;

enum State {
    Start(Option<Colors>),
    White(Colors),
    Black(Colors),
}

struct PossibleWhiteMoves {
    inner: Vec<(usize, usize)>,
}

impl Deref for PossibleWhiteMoves {
    type Target = Vec<(usize, usize)>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PossibleWhiteMoves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

struct PossibleBlackMoves {
    inner: Vec<(usize, usize)>,
}

impl Deref for PossibleBlackMoves {
    type Target = Vec<(usize, usize)>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PossibleBlackMoves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn main() {
    App::new()
        .insert_resource(State::Start(None))
        .insert_resource(PossibleWhiteMoves { inner: Vec::new() })
        .insert_resource(PossibleBlackMoves { inner: Vec::new() })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(create_board)
        .add_startup_system(create_pieces)
        .add_system(mouse_button_input)
        .add_system(move_pieces)
        .add_system(spawn_moves)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

#[allow(clippy::too_many_arguments)]
fn mouse_button_input(
    windows: Res<Windows>,
    camera: Query<&Transform, With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut whites: Query<&mut Piece, (With<WhitePiece>, Without<BlackPiece>)>,
    mut blacks: Query<&mut Piece, (With<BlackPiece>, Without<WhitePiece>)>,
    mut possible_white_moves: ResMut<PossibleWhiteMoves>,
    mut possible_black_moves: ResMut<PossibleBlackMoves>,
    state: ResMut<State>,
    board: Res<Board>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (x, y) = coords(windows, camera);

        match state.as_ref() {
            State::Start(_) if y == 0 => {
                possible_white_moves.clear();

                for i in 1..7 {
                    possible_white_moves.push((x, i));
                }

                for i in 1..=x {
                    if i == 7 {
                        break;
                    }
                    possible_white_moves.push((x - i, i));
                }

                for i in (x + 1)..8 {
                    if i - x == 7 {
                        break;
                    }
                    possible_white_moves.push((i, i - x));
                }

                *state.into_inner() = State::Start(Some(board.tiles()[x][0]));
            }
            State::Start(None) => {}
            State::Start(Some(color)) => {
                if possible_white_moves.contains(&(x, y)) {
                    let mut piece = whites
                        .iter_mut()
                        .find(|piece| piece.color() == *color)
                        .unwrap();

                    piece.move_piece(x, y);
                    possible_white_moves.clear();

                    let mut occupied = [[false; 8]; 8];
                    for white in whites.iter() {
                        occupied[white.x()][white.y()] = true;
                    }
                    for black in blacks.iter() {
                        occupied[black.x()][black.y()] = true;
                    }

                    let black = blacks
                        .iter()
                        .find(|p| p.color() == board.tiles()[x][y])
                        .unwrap();

                    for i in (0..black.y()).rev() {
                        if occupied[black.x()][i] {
                            break;
                        }
                        possible_black_moves.push((black.x(), i))
                    }

                    for i in (0..black.x()).rev() {
                        let y = black.y() - (black.x() - i);
                        if occupied[i][y] {
                            break;
                        }
                        possible_black_moves.push((i, y));
                        if y == 0 {
                            break;
                        }
                    }

                    for i in (black.x() + 1)..8 {
                        let y = black.y() - (i - black.x());
                        if occupied[i][y] {
                            break;
                        }
                        possible_black_moves.push((i, y));
                        if y == 0 {
                            break;
                        }
                    }

                    if !possible_black_moves.is_empty() {
                        *state.into_inner() = State::Black(black.color());
                    }
                }
            }
            State::White(color) => {
                if possible_white_moves.contains(&(x, y)) {
                    let mut piece = whites
                        .iter_mut()
                        .find(|piece| piece.color() == *color)
                        .unwrap();

                    piece.move_piece(x, y);
                    possible_white_moves.clear();

                    let mut occupied = [[false; 8]; 8];
                    for white in whites.iter() {
                        occupied[white.x()][white.y()] = true;
                    }
                    for black in blacks.iter() {
                        occupied[black.x()][black.y()] = true;
                    }

                    let black = blacks
                        .iter()
                        .find(|p| p.color() == board.tiles()[x][y])
                        .unwrap();

                    for i in (0..black.y()).rev() {
                        if occupied[black.x()][i] {
                            break;
                        }
                        possible_black_moves.push((black.x(), i))
                    }

                    for i in (0..black.x()).rev() {
                        let y = black.y() - (black.x() - i);
                        if occupied[i][y] {
                            break;
                        }
                        possible_black_moves.push((i, y));
                        if y == 0 {
                            break;
                        }
                    }

                    for i in (black.x() + 1)..8 {
                        let y = black.y() - (i - black.x());
                        if occupied[i][y] {
                            break;
                        }
                        possible_black_moves.push((i, y));
                        if y == 0 {
                            break;
                        }
                    }

                    if !possible_black_moves.is_empty() {
                        *state.into_inner() = State::Black(black.color());
                    }
                }
            }
            State::Black(color) => {
                if possible_black_moves.contains(&(x, y)) {
                    let mut piece = blacks.iter_mut().find(|p| p.color() == *color).unwrap();

                    piece.move_piece(x, y);
                    possible_black_moves.clear();

                    let mut occupied = [[false; 8]; 8];
                    for white in whites.iter() {
                        occupied[white.x()][white.y()] = true;
                    }
                    for black in blacks.iter() {
                        occupied[black.x()][black.y()] = true;
                    }

                    let white = whites
                        .iter()
                        .find(|p| p.color() == board.tiles()[x][y])
                        .unwrap();

                    for i in (white.y() + 1)..8 {
                        if occupied[white.x()][i] {
                            break;
                        }
                        possible_white_moves.push((white.x(), i));
                    }

                    for i in (0..white.x()).rev() {
                        let y = white.y() + white.x() - i;
                        if occupied[i][y] {
                            break;
                        }
                        possible_white_moves.push((i, y));
                        if y == 7 {
                            break;
                        }
                    }

                    for i in (white.x() + 1)..8 {
                        let y = white.y() + i - white.x();
                        if occupied[i][y] {
                            break;
                        }
                        possible_white_moves.push((i, y));
                        if y == 7 {
                            break;
                        }
                    }

                    if !possible_white_moves.is_empty() {
                        *state.into_inner() = State::White(white.color());
                    }
                }
            }
        }
    }
}

#[derive(Component)]
struct PossibleMove;

fn spawn_moves(
    mut commands: Commands,
    possible_white_moves: Res<PossibleWhiteMoves>,
    possible_black_moves: Res<PossibleBlackMoves>,
    entities: Query<Entity, With<PossibleMove>>,
) {
    if possible_white_moves.is_changed() || possible_black_moves.is_changed() {
        entities.for_each(|entity| commands.entity(entity).despawn());

        let start = -config::BOARD_SIZE / 2.0 + config::TILE_SIZE / 2.0;
        let size = config::TILE_SIZE / 1.5;

        for &(x, y) in possible_white_moves.iter() {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            start + config::TILE_SIZE * x as f32,
                            start + config::TILE_SIZE * y as f32,
                            0.0,
                        ),
                        scale: Vec3::new(size, size, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PossibleMove);
        }

        for &(x, y) in possible_black_moves.iter() {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 0.0, 0.0, 0.8),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            start + config::TILE_SIZE * x as f32,
                            start + config::TILE_SIZE * y as f32,
                            0.0,
                        ),
                        scale: Vec3::new(size, size, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PossibleMove);
        }
    }
}

fn coords(windows: Res<Windows>, camera: Query<&Transform, With<MainCamera>>) -> (usize, usize) {
    let window = windows.get_primary().unwrap();

    let pos = window.cursor_position().unwrap();
    let size = Vec2::new(window.width() as f32, window.height() as f32);
    let p = pos - size / 2.0;
    let camera_transform = camera.single();
    let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

    let start = -config::BOARD_SIZE / 2.0 + config::TILE_SIZE;

    let mut x = 0;
    let mut y = 0;

    for _ in 0..7 {
        if pos_wld.x < start + config::TILE_SIZE * x as f32 {
            break;
        }
        x += 1;
    }

    for _ in 0..7 {
        if pos_wld.y < start + config::TILE_SIZE * y as f32 {
            break;
        }
        y += 1;
    }

    (x, y)
}

fn move_pieces(mut pieces: Query<(&Piece, &mut Transform)>) {
    let start = -config::BOARD_SIZE / 2.0 + config::TILE_SIZE / 2.0;

    for (piece, mut transform) in pieces.iter_mut() {
        transform.translation.x = start + config::TILE_SIZE * piece.x() as f32;
        transform.translation.y = start + config::TILE_SIZE * piece.y() as f32;
    }
}
