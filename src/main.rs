use enigo::*;
use rand::*;
use std::{println, thread, time};

static START_DELAY_VALUE: u64 = 15000;
static FORWARD_BORDER: u64 = 200;

fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    let mut movement_timer: f64 = rng.gen();

    // POV: когда не знаешь приведение типом в языке, на котором пишешь))
    movement_timer = movement_timer * 100 as f64;
    movement_timer = movement_timer.round();
    let num: u64 = movement_timer as u64;
    num
}

fn main() {
    println!("Запущен бот для фарма на koth, пожалуйста, откройте игру в течение 15 секунд");
    let mut enigo = Enigo::new();
    let mut rotation_border_1: i32 = 0;
    let mut rotation_border_2: i32 = 0;

    let start_delay = time::Duration::from_millis(START_DELAY_VALUE);

    thread::sleep(start_delay);
    enigo.key_down(Key::Shift);
    let mut is_right_direction: bool = generate_random_number() % 2 == 0;

    while true {
        // В loop не работает нажатие на клавиатуру. ???
        let cool_down = time::Duration::from_millis(generate_random_number() as u64 * 1000);

        let mut forward_step: u64 = generate_random_number(); // получаем рандомное число
        if rotation_border_1 < 300 {
            rotation_border_1 = rotation_border_1 + 1;

            forward_step = forward_step + 1;
            if forward_step < FORWARD_BORDER {
                // Так мы каждый раз бежим вперед разное количество времени
                enigo.key_click(Key::Layout('w'));
            }

            if is_right_direction {
                // Иногда крутимся по часовой, иногда против
                enigo.mouse_move_relative(generate_random_number() as i32, 6);
            } else {
                enigo.mouse_move_relative(-1 * generate_random_number() as i32, 6);
            }
        } else if rotation_border_2 < 300 {
            // После того как выполнили норму стометровки вперед, выполняем норму бега назад
            rotation_border_2 = rotation_border_2 + 1;
            enigo.key_click(Key::Layout('s'));
            enigo.mouse_move_relative(-1 * generate_random_number() as i32, -6);
        } else {
            // Зануляем наши перменные перед новой итерации
            rotation_border_1 = 0;
            rotation_border_2 = 0;
            enigo.key_up(Key::Shift);
            println!("Следующая итерация через: {:?}", &cool_down);
            thread::sleep(cool_down); // Ждем некоторое время

            // Тут решается, в какую сторону мы будем крутиться в след итерации
            is_right_direction = generate_random_number() % 2 == 0;
        }
    }
}
