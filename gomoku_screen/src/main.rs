// macroquadという名前のゲーム作成用道具箱を使う
use macroquad::prelude::*;

const BOARD_SIZE: usize = 19;
const CELL_SIZE: f32 = 30.0;
const OFFSET: f32 = 40.0;
// このプログラムはmacroquadというライブライを使って動かし、立ち上がったウィンドウでgomokuと表示する
#[macroquad::main("Gomoku")]
// 一回ループするごとに「画面を消すー＞線を引くー＞石を描くという動作を繰り返す
async fn main() {
    let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
    loop {
        clear_background(BEIGE);
        for i in 0..19 {
            let pos = OFFSET + i as f32 * CELL_SIZE;
            draw_line(OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, pos ,1.0, BLACK);
            draw_line(pos, OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, 1.0, BLACK);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let x = ((mx - OFFSET + 15.0) / CELL_SIZE) as i32;
            let y = ((my - OFFSET + 15.0) / CELL_SIZE) as i32;
            
            if x >= 0 && x < 19 && y >= 0 && y < 19 {
                board[y as usize][x as usize] = 1;
            }
        }
        for y in 0..19 {
            for x in 0..19 {
                if board[y][x] == 1 {
                    draw_circle(OFFSET + x as f32 * CELL_SIZE, OFFSET + y as f32 * CELL_SIZE, 13.0, BLACK);
                }
            }
        }
        next_frame().await

    }
}