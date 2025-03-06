use std::fs::File;
use std::io::Read;
use std::process::exit;

enum States {
    S(String), // Начальное
    K(String), // Конечное Удалить?
    E(String), // Ошибка
    Sn(String),// Север
    Ss(String),// Юг
    Sw(String),// Запад
    Se(String) // Восток
}

// fn next_state(ch:char, state:char)
// {

// }

fn main() {
    let mut code = File::open("./code.txt").unwrap(); // ? - если ошибка выход из функции
    let mut ch = [0u8; 1];
    let mut state: States = States::S("Start".to_string());

    while code.read(&mut ch).unwrap() != 0 
    {
        let ch = ch[0] as char; // Преобразуем байт в char

        match ch {
            ';' => {
                match state {
                    States::Se(value) => {
                        println!(" Program correct. Robot look at {}", value);
                        state = States::K("End state".to_string()); // Удалить ?
                        exit(0);
                    },
                    States::Sn(value) => {
                        println!(" Program correct. Robot look at {}", value);
                        state = States::K("End state".to_string()); // Удалить ?
                        exit(0);
                    },
                    States::Sw(value) => {
                        println!(" Program correct. Robot look at {}", value);
                        state = States::K("End state".to_string()); // Удалить ?
                        exit(0);
                    },
                    States::Ss(value) => {
                        println!(" Program correct. Robot look at {}", value);
                        state = States::K("End state".to_string()); // Удалить ?
                        exit(0);
                    },
                    // Остальные состояния
                    _ =>
                    {
                        println!(" Program incorrect");
                        state = States::E("Error".to_string());
                        exit(1);
                    }
                }
            },
            'N' => {
                match state {
                    States::S(value) =>{
                        state = States::Sn("North".to_string());
                    },
                    // Остальные состояния
                    _ =>
                    {
                        println!(" Program incorrect.");
                        state = States::E("Error".to_string()); // Удалить ?
                        exit(1);
                    }
                }
            },
            'S' => {
                match state {
                    States::S(value) =>{
                        state = States::Sn("South".to_string());
                    },
                    // Остальные состояния
                    _ =>
                    {
                        println!(" Program incorrect.");
                        state = States::E("Error".to_string()); // Удалить ?
                        exit(1);
                    }
                }
            },
            'W' => {
                match state {
                    States::S(value) =>{
                        state = States::Sn("West".to_string());
                    },
                    // Остальные состояния
                    _ =>
                    {
                        println!(" Program incorrect.");
                        state = States::E("Error".to_string()); // Удалить ?
                        exit(1);
                    }
                }
            },
            'E' => {
                match state {
                    States::S(value) =>{
                        state = States::Sn("East".to_string());
                    },
                    // Остальные состояния
                    _ =>
                    {
                        println!(" Program incorrect.");
                        state = States::E("Error".to_string()); // Удалить ?
                        exit(1);
                    }
                }
            },
            'r' => {
                match state {
                    // Север -> Восток
                    States::Sn(value) =>
                    {
                        state = States::Se("East".to_string()); 
                    },
                    // Юг -> Запад
                    States::Ss(value) =>
                    {
                        state = States::Sw("West".to_string());
                    },
                    // Запад -> Север
                    States::Sw(value) =>
                    {
                        state = States::Sn("North".to_string());
                    },
                    // Восток -> Юг
                    States::Se(value) =>
                    {
                        state = States::Ss("South".to_string());
                    },
                    // Остальные состояния
                    _ =>{
                        println!(" Program incorrect.");
                        state = States::E("Error".to_string()); // Удалить ?
                        exit(1);                    
                    }
                }
            },
            'l' => {
                match state {
                    // Север -> Запад
                    States::Sn(value) =>
                    {
                        state = States::Sw("West".to_string()); 
                    },
                    // Юг -> Восток
                    States::Ss(value) =>
                    {
                        state = States::Se("East".to_string()); 
                    },
                    // Запад -> Юг
                    States::Sw(value) =>
                    {
                        state = States::Ss("South".to_string()); 
                    },
                    // Восток -> Север
                    States::Se(value) =>
                    {
                        state = States::Sn("North".to_string()); 
                    },
                    // Остальные состояния
                    _ =>{
                        println!(" Program incorrect.");
                        state = States::E("Error".to_string()); // Удалить ?
                        exit(1);                    
                    }
                }
            },
            'f' => {
                match state {
                    // Север -> Север
                    States::Sn(value) =>
                    {
                        state = States::Sn("North".to_string()); 
                    },
                    // Юг -> Юг
                    States::Ss(value) =>
                    {
                        state = States::Ss("South".to_string()); 
                    },
                    // Запад -> Запад
                    States::Sw(value) =>
                    {
                        state = States::Sw("West".to_string()); 
                    },
                    // Восток -> Восток
                    States::Se(value) =>
                    {
                        state = States::Se("East".to_string()); 
                    },
                    // Остальные состояния
                    _ =>{
                        println!(" Program incorrect.");
                        state = States::E("Error".to_string()); // Удалить ?
                        exit(1);                    
                    }
                }
            },
            _ => {
                println!(" Program incorrect. Unrecognized symbol appear");
                state = States::E("Error".to_string());
                exit(1);
            }
        }
    }

        println!(" Program incorrect. Not enough end symbol ';'");
        state = States::E("Error".to_string());
        exit(1);
}
