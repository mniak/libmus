extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

enum Nota {
    Pausa,
    Nota { Duracao: Figura },
}

type Altura = u8;
enum Figura {
    Semibreve,
    Minima,
    Seminima,
    Colcheia,
    Semicolcheia,
}
enum Bandeira {
    Nenhuma,
    Colcheia,
    Semicolcheia,
}
enum Desenho {
    Haste { Bandeira: Bandeira },
    Bolinha { Duracao: Figura, Altura: Altura },
}

struct Grupo {
    Notas: Vec<Nota>,
}

enum Item {
    Grupo,
    Pausa { Figura: Figura },
}

struct Pauta {
    Armadura: u8,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Musigym", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_bravura = ttf_context.load_font("../fonts/Bravura.otf", 16);
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
