use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let scale_factor = 5;

    let mut event_pump = sdl_context.event_pump()?;

    let window = sdl_context
        .video()?
        .window(
            "title: Gameboy",
            (160 * scale_factor) as u32,
            (144 * scale_factor) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync() // TODO: this work?
        .build()
        .map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture(
            sdl2::pixels::PixelFormatEnum::RGB24,
            sdl2::render::TextureAccess::Static,
            3,
            1,
        )
        .unwrap();

    texture
        .update(None, &[255, 255, 255, 0, 0, 0, 255, 255, 255], 9)
        .unwrap();

    canvas.copy(&texture, None, None)?;
    canvas.present();

    'main: loop {
        // Handle events to close window.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    texture
                        .update(None, &[0, 0, 0, 255, 255, 255, 0, 0, 0], 9)
                        .unwrap();
                    canvas.copy(&texture, None, None)?;
                    canvas.present();
                }
                _ => (),
            }
        }
    }

    Ok(())
}
