pub mod render;


pub struct Engine {
	sdl3_ins: sdl3::Sdl,
	render_engine: render::RenderEngine,
}

impl Engine {
    pub fn init() -> Self {
		let sdl3_err: Result<sdl3::Sdl, sdl3::Error> = sdl3::init();
		if sdl3_err.is_err() {
			panic!("Failed to initialize SDL3: {:?}", sdl3_err.err());
		}

		let sdl3_ins: sdl3::Sdl = sdl3_err.unwrap();
		
		
		let render_engine: render::RenderEngine = render::RenderEngine {};
		render_engine.make_window("InventoryCraft", 1920, 1080, &sdl3_ins);

        Engine { sdl3_ins, render_engine }
    }
	pub fn is_running(&self) -> bool {
        true
    }
}