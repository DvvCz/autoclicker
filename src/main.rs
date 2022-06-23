mod util;

use mouse_rs::{Mouse, types::keys::Keys as MouseButton};
use device_query::{DeviceState, Keycode, DeviceEvents};
use colored::Colorize;

use std::{
	time::Duration,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering}
	}
};

/// How often to check if we've started autoclicking in the autoclick thread.
static THREAD_TIME: Duration = Duration::from_millis(300);
static CLICK_DELAY: Duration = Duration::from_millis(100);

#[inline]
fn is_in_game() -> bool {
	util::get_active_window().contains("Minecraft")
}

fn main() {
	let state = DeviceState::new();
	let autoclicking = Arc::new( AtomicBool::new(false) );

	let autoclicking_thread = Arc::clone(&autoclicking);
	std::thread::spawn(move || {
		let autoclicking = autoclicking_thread;
		let mouse = Mouse::new();

		loop {
			while autoclicking.load(Ordering::Relaxed) {
				if !is_in_game() {
					autoclicking.store(false, Ordering::Relaxed);
					println!("{}", "Disabled autoclicker. You tabbed out".red());
					break;
				}

				let _ = mouse.click(&MouseButton::LEFT);
				std::thread::sleep(CLICK_DELAY);
			}

			std::thread::sleep(THREAD_TIME);
		}
	});

	// Make sure not to discard this callback guard (or else it won't work since rust will drop it immediately.)
	let _guard = state.on_key_down(move |key| {
		match key {
			Keycode::LShift => {
				let is_autoclicking = !autoclicking.load(Ordering::Acquire);

				if is_in_game() {
					// Was disabled previously, and the current window is in game.
					autoclicking.store(is_autoclicking, Ordering::Release);

					let msg = if is_autoclicking {
						"Clicking".green()
					} else {
						"Stopped clicking".red()
					};

					println!("{msg}");
				} else if is_autoclicking {
					// Trying to enable autoclicking outside of game.
					println!("{}", "Prevented autoclick while outside of game.".yellow())
				}
			},
			Keycode::Equal => {
				autoclicking.store(false, Ordering::Release);

				#[cfg(feature = "sound")]
				{
					let _ = util::play_sound( include_bytes!("../assets/shutdown.mp3") );
					std::thread::sleep( Duration::from_secs(2) );
				}

				std::process::exit(0)
			},
			_ => ()
		}
	});

	loop {}
}
