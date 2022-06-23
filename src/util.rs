/// Does not work right now.
pub fn get_active_window<'a>() -> String {
	let mut title: Vec<u8> = vec![0; 100];
	unsafe {
		let window = winapi::um::winuser::GetForegroundWindow();
		winapi::um::winuser::GetWindowTextA(window, title.as_mut_ptr().cast(), 100);
	}

	String::from_utf8_lossy(&title).to_string()
}


#[cfg(feature = "sound")]
use rodio::{
	PlayError,
	OutputStream,
	Decoder,
	Source
};

#[cfg(feature = "sound")]
pub fn play_sound(bytes: &'static [u8]) -> Result<(), PlayError> {
	use std::io::Cursor;

	let (_, stream_handle) = OutputStream::try_default().unwrap();

	let file = Cursor::new(bytes);
	let source = Decoder::new(file).unwrap();

	stream_handle.play_raw(source.convert_samples())?;

	Ok(())
}