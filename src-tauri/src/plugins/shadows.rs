use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
};

pub fn set_shadow(
	window: impl raw_window_handle::HasRawWindowHandle,
	enable: bool,
) -> Result<(), Error> {
	match window.raw_window_handle() {
		#[cfg(target_os = "macos")]
		raw_window_handle::RawWindowHandle::AppKit(handle) => {
			use cocoa::{appkit::NSWindow, base::id};
			use objc::runtime::{NO, YES};

			unsafe {
				(handle.ns_window as id).setHasShadow_(if enable { YES } else { NO });
			}

			Ok(())
		}
		#[cfg(target_os = "windows")]
		raw_window_handle::RawWindowHandle::Win32(handle) => {
			use windows_sys::Win32::{
				Graphics::Dwm::DwmExtendFrameIntoClientArea, UI::Controls::MARGINS,
			};

			let m = if enable { 1 } else { 0 };
			let margins = MARGINS {
				cxLeftWidth: m,
				cxRightWidth: m,
				cyTopHeight: m,
				cyBottomHeight: m,
			};

			unsafe {
				DwmExtendFrameIntoClientArea(handle.hwnd as _, &margins);
			}
			Ok(())
		}
		_ => Err(Error::UnsupportedPlatform),
	}
}

#[derive(Debug)]
pub enum Error {
	UnsupportedPlatform,
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("\"set_shadow()\" is only supported on Windows and macOS")
	}
}

impl StdError for Error {}
