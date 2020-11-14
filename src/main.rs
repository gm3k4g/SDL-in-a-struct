use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
 
pub struct SdlWindow {
	pub sdl_context: sdl2::Sdl,
	pub video_subsystem: sdl2::VideoSubsystem,
	pub canvas: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,

}
impl SdlWindow {
	fn new(title: &str, width: u32, height: u32) -> Self {


	let sdlc = sdl2::init().unwrap();
    let videos = sdlc.video().unwrap();
 
 	// Consume Window to create WindowCanvas
    let win = videos.window(title, width, height)
        .position_centered()
        .build()
        .unwrap();
    let canv = win.into_canvas().build().unwrap();

    let ep = sdlc.event_pump().unwrap();

		SdlWindow {
			sdl_context: sdlc,
			video_subsystem: videos,
       		canvas: canv,
       		event_pump: ep,
		}
	}

	fn prepare_canvas(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 255, 255));
    	self.canvas.clear();
    	self.canvas.present();
	}

	fn main_loop(&mut self) {
	    let mut i = 0;
	    'running: loop {
	        i = (i + 1) % 255;
	        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
	        self.canvas.clear();
	        for event in self.event_pump.poll_iter() {
	            match event {
	                Event::Quit {..} |
	                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
	                    break 'running
	                },
	                _ => {}
	            }
	        }
	        // The rest of the game loop goes here...

	        self.canvas.present();
	        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	    }
	}
}

fn main() {
	let mut sdlwindow = SdlWindow::new("Hello", 800, 600);
	sdlwindow.prepare_canvas();
	sdlwindow.main_loop();
}