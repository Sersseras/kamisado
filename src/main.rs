use bevy::{
    input::Input,
    math::Vec2,
    prelude::{
        App, Commands, Component, MouseButton, OrthographicCameraBundle, Query, Res, ResMut,
        Transform, With, Without,
    },
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
    Start,
    White(Colors),
    Black(Colors),
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(create_board)
        .add_startup_system(create_pieces)
        .add_system(mouse_button_input)
        .add_system(move_pieces)
        .insert_resource(State::Start)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn mouse_button_input(
    windows: Res<Windows>,
    camera: Query<&Transform, With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut whites: Query<&mut Piece, (With<WhitePiece>, Without<BlackPiece>)>,
    mut blacks: Query<&mut Piece, (With<BlackPiece>, Without<WhitePiece>)>,
    state: ResMut<State>,
    board: Res<Board>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (x, y) = coords(windows, camera);

        match state.as_ref() {
            State::Start => {
                let mut piece = whites.iter_mut().next().unwrap();

                piece.move_piece(0, 1);

                *state.into_inner() = State::Black(board.tiles()[0][1])
            }
            State::White(color) => {
                let mut occupied = [[false; 8]; 8];

                for white in whites.iter() {
                    occupied[white.x()][white.y()] = true;
                }

                for black in blacks.iter() {
                    occupied[black.x()][black.y()] = true;
                }

                let mut piece = whites
                    .iter_mut()
                    .find(|piece| piece.color() == *color)
                    .unwrap();

                piece.move_piece(0, 1);

                *state.into_inner() = State::Black(board.tiles()[0][1])
            }
            State::Black(color) => {
                let mut occupied = [[false; 8]; 8];

                for white in whites.iter() {
                    occupied[white.x()][white.y()] = true;
                }

                for black in blacks.iter() {
                    occupied[black.x()][black.y()] = true;
                }

                let mut piece = blacks
                    .iter_mut()
                    .find(|piece| piece.color() == *color)
                    .unwrap();

                piece.move_piece(0, 1);
                *state.into_inner() = State::White(board.tiles()[0][1])
            }
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
