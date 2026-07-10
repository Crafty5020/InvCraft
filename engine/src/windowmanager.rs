use common::ERROR;

pub struct WindowManager {
	windows: Vec<sdl3::video::Window>,
}

impl WindowManager {
	pub fn init(window_name: &str, width: u32, height: u32, sdl: &sdl3::Sdl) -> Result<Self, common::ERROR> {
		let window_check: Result<(), common::ERROR> = Self::create_window(&mut WindowManager { windows: Vec::new() }, window_name, 1920, 1080, sdl);
		if window_check.is_err() {
			return Err(window_check.err().unwrap());
		}
		Ok(WindowManager { windows: Vec::new() })
	}
	pub fn create_window(&mut self, title: &str, width: u32, height: u32, sdl: &sdl3::Sdl) -> Result<(), ERROR> {
		let video_subsystem: Result<sdl3::VideoSubsystem, sdl3::Error> = sdl.video();
		if video_subsystem.is_err() {
			println!("Failed to initialize video subsystem: {:?}", video_subsystem.err());
			return Err(ERROR::SdlVIDEOSUBInitFailed);
		}
		let video_subsystem: sdl3::VideoSubsystem = video_subsystem.unwrap();

		let window = video_subsystem
			.window(title, width, height)
			.position_centered()
			.build()
			.map_err(|_| ERROR::SdlWindowCreationFailed)?;

		self.windows.push(window);

		Ok(())
	}
}