extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{self, Rect};
use sdl2::render::Texture;
use std::time::Duration;

enum Notehead {
    Whole,
    Half,
    Black,
}

impl From<&Notehead> for char {
    fn from(value: &Notehead) -> Self {
        match value {
            Notehead::Whole => '\u{E0A2}',
            Notehead::Half => '\u{E0A3}',
            Notehead::Black => '\u{E0A4}',
        }
    }
}

enum Staff {
    FiveLinesWide,
    FiveLinesNarrow,
}

impl From<&Staff> for char {
    fn from(value: &Staff) -> Self {
        match value {
            Staff::FiveLinesWide => '\u{E01A}',
            Staff::FiveLinesNarrow => '\u{E020}',
        }
    }
}

enum Glyph {
    Staff(Staff),
    Notehead(Notehead),
}

impl From<&Glyph> for char {
    fn from(value: &Glyph) -> Self {
        match value {
            Glyph::Staff(staff) => char::from(staff),
            Glyph::Notehead(notehead) => char::from(notehead),
        }
    }
}

struct Nota {
    Head: Notehead,
    Altura: u8,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Musigym", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init()
        .map_err(|e| println!("sdl2_ttf init error: {:?}", e))
        .unwrap();
    let font_bravura = ttf_context
        .load_font("./fonts/BravuraText.otf", 72)
        .map_err(|e| println!("Font loading error: {:?}", e))
        .unwrap();
    let mut canvas = window
        .into_canvas()
        // .software()
        .build()
        .map_err(|e| println!("Windows into canvas error: {:?}", e))
        .unwrap();
    let mut event_pump = sdl_context
        .event_pump()
        .map_err(|e| println!("Event pump creation error: {:?}", e))
        .unwrap();
    let texture_creator = canvas.texture_creator();

    let mut i = 0;
    'running: loop {
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

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        canvas.set_draw_color(Color::BLUE);
        canvas.fill_rect(Rect::new(10, 20, 40, 70)).unwrap();

        let surf = font_bravura
            .render(
                [
                    Glyph::Staff(Staff::FiveLinesWide),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Whole),
                    Glyph::Notehead(Notehead::Half),
                    Glyph::Staff(Staff::FiveLinesWide),
                    Glyph::Notehead(Notehead::Black),
                ]
                .iter()
                .map(|n| char::from(n))
                .collect::<String>()
                .as_str(),
            )
            .blended(Color::RED)
            .map_err(|e| println!("Font rendering error: {:?}", e))
            .unwrap();
        let tex = texture_creator
            .create_texture_from_surface(&surf)
            .map_err(|e| println!("Texture from surface error: {:?}", e))
            .unwrap();

        let src_rect = Rect::new(0, 0, surf.width(), surf.height());
        let target_rect = Rect::new(100, 100, surf.width(), surf.height());
        canvas.set_draw_color(Color::YELLOW);
        canvas
            .fill_rect(target_rect)
            .map_err(|e| println!("Fill rect yellow error: {:?}", e))
            .unwrap();
        canvas
            .copy(&tex, src_rect, target_rect)
            .map_err(|e| println!("Canvas copy error: {:?}", e))
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(100));
    }
}
