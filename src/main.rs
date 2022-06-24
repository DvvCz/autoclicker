mod util;
mod mouse;

use mouse::{Mouse, Button as MouseButton};

use device_query::{DeviceState, Keycode, DeviceQuery};
use colored::Colorize;

use std::{
	time::Duration,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering}
	}
};

/// How often to check if we've started autoclicking in the autoclick thread.
const THREAD_TIME: Duration = Duration::from_millis(300);
const CLICK_DELAY: Duration = Duration::from_millis(700);
const CHECK_TIME: Duration = Duration::from_millis(50);

#[inline]
fn is_in_game() -> bool {
	util::get_active_window().contains("Minecraft")
}
fn main() {
	let state = DeviceState::new();
	let mut autoclicking = Arc::new( AtomicBool::new(false) );

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

				mouse.click(MouseButton::LEFT);
				std::thread::sleep(CLICK_DELAY);
			}

			std::thread::sleep(THREAD_TIME);
		}
	});

	let mut last = None;

	fn handle_key(key: Keycode, autoclicking: &mut Arc<AtomicBool>) {
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
	}

	loop {
		let keys = state.get_keys();

		match last {
			None => {
				last = Some(keys);
			},
			Some(ref last_inner) if last_inner == &keys => (),
			Some(ref last_inner) => {
				let mut diff = keys.clone();
				diff.retain(|x| !last_inner.contains(x));

				for k in diff {
					handle_key(k, &mut autoclicking);
				}

				last = Some(keys);
			}
		}

		std::thread::sleep(CHECK_TIME);
	}
}
