pub mod render;
pub mod windowmanager;

use common::ERROR;

pub struct Engine {
	sdl3_ins: sdl3::Sdl,
	render_engine: render::RenderEngine,
	window_manager: windowmanager::WindowManager,
}

impl Engine {
    pub fn init(window_name: &str, width: u32, height: u32) -> Result<Self, ERROR> {
		let sdl3_err: Result<sdl3::Sdl, sdl3::Error> = sdl3::init();
		if sdl3_err.is_err() {
			return Err(ERROR::SdlInitFailed);
		}

		let sdl3_ins: sdl3::Sdl = sdl3_err.unwrap();
		
		
		let render_engine: render::RenderEngine = render::RenderEngine::init().unwrap();
		let window_manager: Result<windowmanager::WindowManager, ERROR> = windowmanager::WindowManager::init(window_name, 1920, 1080, &sdl3_ins);

		if window_manager.is_err() {
			return Err(window_manager.err().unwrap());
		}

        return Ok(Engine { sdl3_ins, render_engine, window_manager: window_manager.unwrap() });
    }
	pub fn is_running(&self) -> bool {
        true
    }
	pub fn stop(&self) {
		println!("Stopping engine.......................................................CAINE WAS HERE WITH BUBBLE...............");
		
	}
}