use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
 
pub struct SdlContext {
	pub sdl_context: sdl2::Sdl,
}
impl SdlContext {
	fn new() -> Self {
		SdlContext {
			sdl_context: sdl2::init().unwrap(),
		}
	}
}

pub struct SdlVideoSubsystem {
	pub video_subsystem: sdl2::VideoSubsystem,
}
impl SdlVideoSubsystem {
	fn new(sdlcontext: sdl2::Sdl) -> Self {
		SdlVideoSubsystem {
			video_subsystem: sdlcontext.video().unwrap(),
		}
	}
}

pub struct SdlCanvas {
	pub canvas: sdl2::render::WindowCanvas,
}
impl SdlCanvas {
	fn new(title: &str, width: u32, height: u32, sdlvideo: sdl2::VideoSubsystem) -> Self {
	    let win = sdlvideo.window(title, width, height)
	        .position_centered()
	        .build()
	        .unwrap();
	    let canv = win.into_canvas().build().unwrap();
	    SdlCanvas {
	    	canvas: canv,
	    }
	}

	fn prepare(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 255, 255));
    	self.canvas.clear();
    	self.canvas.present();
	}
}

pub struct SdlEventPump {
	pub event_pump: sdl2::EventPump,
}
impl SdlEventPump {
	fn new(sdlcontext: sdl2::Sdl) -> Self {
		SdlEventPump {
			event_pump: sdlcontext.event_pump().unwrap(),
		}
	}
}

pub struct SdlLoop {
	pub sdl_context: sdl2::Sdl,
	pub video_subsystem: sdl2::VideoSubsystem,
	pub canvas: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,
}
impl SdlLoop {
	fn new(mut canvas: sdl2::render::WindowCanvas, mut event_pump: sdl2::EventPump) {
	    let mut i = 0;
	    'running: loop {
	        i = (i + 1) % 255;
	        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
	        canvas.clear();
	        for event in event_pump.poll_iter() {
	            match event {
	                Event::Quit {..} |
	                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
	                    break 'running
	                },
	                _ => {}
	            }
	        }
	        // The rest of the game loop goes here...

	        canvas.present();
	        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	    }
	}
}

fn main() {
	let sdlcontext 					= SdlContext::new();
	let sdlvideo 					= SdlVideoSubsystem::new(sdlcontext.sdl_context); 
	let /*mut*/ sdl_event_pump 	    = SdlEventPump::new(sdlvideo.video_subsystem.sdl());
	let mut sdlcanvas 				= SdlCanvas::new("Hello World", 800, 600, sdlvideo.video_subsystem);
	sdlcanvas.prepare();

	SdlLoop::new(sdlcanvas.canvas, sdl_event_pump.event_pump);
}
