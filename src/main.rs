use bevy::{
    input::Input,
    prelude::{
        App, Commands, MouseButton, OrthographicCameraBundle, Query, Res, Transform, With, Without,
    },
    DefaultPlugins,
};
use board::create_board;
use pieces::{create_pieces, BlackPiece, Piece, WhitePiece};

mod board;
mod colors;
mod config;
mod pieces;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(create_board)
        .add_startup_system(create_pieces)
        .add_system(mouse_button_input)
        .add_system(move_pieces)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    mut whites: Query<&mut Piece, (With<WhitePiece>, Without<BlackPiece>)>,
    mut blacks: Query<&mut Piece, (With<BlackPiece>, Without<WhitePiece>)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for mut white in whites.iter_mut() {
            let x = white.x();
            let y = white.y() + 1;
            white.move_piece(x, y);
        }

        for mut black in blacks.iter_mut() {
            let x = black.x();
            let y = black.y() - 1;
            black.move_piece(x, y);
        }
    }
}

fn move_pieces(mut pieces: Query<(&Piece, &mut Transform)>) {
    let tile_size = config::BOARD_SIZE / 8.0;
    let start = -config::BOARD_SIZE / 2.0 + tile_size / 2.0;

    for (white, mut transform) in pieces.iter_mut() {
        transform.translation.x = start + tile_size * white.x() as f32;
        transform.translation.y = start + tile_size * white.y() as f32;
    }
}
