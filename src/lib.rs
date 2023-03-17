#![no_std]

use core::{arch::wasm32, panic::PanicInfo};

const GAMEPAD1: *const u8 = 0x16 as *const u8;

const BUTTON_LEFT: u8 = 16;
const BUTTON_RIGHT: u8 = 32;
const BUTTON_UP: u8 = 64;
const BUTTON_DOWN: u8 = 128;

const STEP_SIZE: f32 = 0.045;

extern "C" {
    fn vline(x: i32, y: i32, len: u32);
}

const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}    

impl State {
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
        let previous_position = (self.player_x, self.player_y);

        if up {
            self.player_x += cosf(self.player_angle) * STEP_SIZE;
            self.player_y += -sinf(self.player_angle) * STEP_SIZE;
        }

        if down {
            self.player_x -= cosf(self.player_angle) * STEP_SIZE;
            self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
        }

        if right {
            self.player_angle -= STEP_SIZE;
        }

        if left {
            self.player_angle += STEP_SIZE;
        }

        if point_in_wall(self.player_x, self.player_y) {
            (self.player_x, self.player_y) = previous_position;
        }
    }
}

static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
};

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

#[no_mangle]
unsafe fn update() {
    if *GAMEPAD1 & BUTTON_UP != 0 {
        vline(80, 20, 120);
    }
}

/// Check if the map contains a wall at a point.
fn point_in_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize)) != 0,
        None => true,
    }
}

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/