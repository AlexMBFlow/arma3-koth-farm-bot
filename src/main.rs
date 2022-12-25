use enigo::*;
use rand::*;
use std::{thread, time};

fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    let mut movement_timer: f64 = rng.gen();
    movement_timer = movement_timer * 100 as f64;
    movement_timer = movement_timer.round();
    let num: u64 = movement_timer as u64;
    return num;
}

fn main() {
    let mut enigo = Enigo::new();
    let mut rotation_border_1: i32 = 0;
    let mut rotation_border_2: i32 = 0;

    let start_delay = time::Duration::from_millis(5000);

    thread::sleep(start_delay);
    enigo.key_down(Key::Shift);
    let mut is_right_direction: bool = generate_random_number() % 2 == 0;

    while true {
        let cool_down = time::Duration::from_millis(generate_random_number() as u64 * 1000);

        let mut forward_border: u64 = 200;
        let forward_step: u64 = generate_random_number();
        if rotation_border_1 < 300 {
            rotation_border_1 = rotation_border_1 + 1;

            forward_border = forward_border + 1;
            if forward_step < forward_border {
                enigo.key_click(Key::Layout('w'));
            }

            if is_right_direction {
                enigo.mouse_move_relative(generate_random_number() as i32, 6);
            } else {
                enigo.mouse_move_relative(-1 * generate_random_number() as i32, 6);
            }
        } else if rotation_border_2 < 300 {
            rotation_border_2 = rotation_border_2 + 1;
            enigo.key_click(Key::Layout('s'));

            enigo.mouse_move_relative(-1 * generate_random_number() as i32, -6);
        } else {
            rotation_border_1 = 0;
            rotation_border_2 = 0;
            enigo.key_up(Key::Shift);
            thread::sleep(cool_down);
            is_right_direction = generate_random_number() % 2 == 0;
            //println!("{:?}", cool_down);
            //cool_down = time::Duration::from_millis(generate_random_number() as u64);
        }
    }
}
