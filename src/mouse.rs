use std::mem::MaybeUninit;

use winapi::um::winuser::{SendInput, INPUT, INPUT_MOUSE, MK_LBUTTON, GetMessageExtraInfo, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEUP};

pub struct Mouse;

pub enum Button {
	LEFT,
	RIGHT,
	MIDDLE
}

impl Mouse {
	pub fn new() -> Self {
		Self
	}

	fn input() -> INPUT {
		unsafe {
			INPUT {
				type_: INPUT_MOUSE,
				u: MaybeUninit::zeroed().assume_init()
			}
		}
	}

	/// Sends a list of inputs and returns if successful
	#[must_use]
	fn send(count: u32, inputs: &mut INPUT) -> bool {
		unsafe {
			SendInput(count, inputs, std::mem::size_of::<INPUT>() as _) == count
		}
	}

	pub fn click(&self, btn: Button) -> bool {
		let mut input = Self::input();

		unsafe {
			// https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-mouseinput
			input.u.mi_mut().dwFlags = match btn {
				Button::LEFT => MOUSEEVENTF_LEFTDOWN,
				Button::RIGHT => MOUSEEVENTF_RIGHTDOWN,
				Button::MIDDLE => MOUSEEVENTF_MIDDLEDOWN
			};

			if !Self::send(1, &mut input) {
				return false;
			}

			input.u.mi_mut().dwFlags = match btn {
				Button::LEFT => MOUSEEVENTF_LEFTUP,
				Button::RIGHT => MOUSEEVENTF_RIGHTUP,
				Button::MIDDLE => MOUSEEVENTF_MIDDLEUP
			};
		}

		Self::send(1, &mut input)
	}
}