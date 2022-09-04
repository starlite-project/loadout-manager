use std::{
	borrow::Cow,
	fmt::Arguments,
	fs::{self, File},
	iter::FromIterator,
	path::{Path, PathBuf},
};

pub use fern;
use fern::FormatCallback;
use log::{LevelFilter, Record};
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tauri::{
	plugin::{self, TauriPlugin},
	Manager, Runtime,
};

const DEFAULT_MAX_FILE_SIZE: u128 = 40000;
const DEFAULT_ROTATION_STRATEGY: RotationStrategy = RotationStrategy::KeepOne;
const DEFAULT_LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stderr, LogTarget::LogDir];

#[repr(u16)]
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
pub enum LogLevel {
	Trace = 1,
	Debug,
	Info,
	Warn,
	Error,
}

impl From<LogLevel> for log::Level {
	fn from(log_level: LogLevel) -> Self {
		match log_level {
			LogLevel::Trace => Self::Trace,
			LogLevel::Debug => Self::Debug,
			LogLevel::Info => Self::Info,
			LogLevel::Warn => Self::Warn,
			LogLevel::Error => Self::Error,
		}
	}
}

impl From<log::Level> for LogLevel {
	fn from(log_level: log::Level) -> Self {
		match log_level {
			log::Level::Trace => Self::Trace,
			log::Level::Debug => Self::Debug,
			log::Level::Info => Self::Info,
			log::Level::Warn => Self::Warn,
			log::Level::Error => Self::Error,
		}
	}
}

impl From<LogLevel> for log::LevelFilter {
	fn from(log_level: LogLevel) -> Self {
		match log_level {
			LogLevel::Trace => Self::Trace,
			LogLevel::Debug => Self::Debug,
			LogLevel::Info => Self::Info,
			LogLevel::Warn => Self::Warn,
			LogLevel::Error => Self::Error,
		}
	}
}

impl From<log::LevelFilter> for LogLevel {
	fn from(log_level: log::LevelFilter) -> Self {
		match log_level {
			log::LevelFilter::Trace => Self::Trace,
			log::LevelFilter::Debug => Self::Debug,
			log::LevelFilter::Info => Self::Info,
			log::LevelFilter::Warn => Self::Warn,
			log::LevelFilter::Error | log::LevelFilter::Off => Self::Error,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum RotationStrategy {
	KeepAll,
	KeepOne,
}

#[derive(Debug, Clone, Serialize)]
struct RecordPayload {
	message: String,
	level: LogLevel,
}

#[derive(Debug, Clone)]
pub enum LogTarget {
	Stdout,
	Stderr,
	Folder(PathBuf),
	LogDir,
	Webview,
}

#[tauri::command]
fn log(level: LogLevel, message: String, location: Option<&str>) {
	let location = location.unwrap_or("webview");

	log::log!(target: location, level.into(), "{}", message);
}

pub struct LoggerBuilder {
	dispatch: fern::Dispatch,
	rotation_strategy: RotationStrategy,
	max_file_size: u128,
	targets: Vec<LogTarget>,
}

impl LoggerBuilder {
	pub fn new() -> Self {
		let dispatch = fern::Dispatch::new().format(move |out, message, record| {
			out.finish(format_args!(
				"{}[{}][{}] {}",
				chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
				record.target(),
				record.level(),
				message
			))
		});

		Self {
			dispatch,
			rotation_strategy: DEFAULT_ROTATION_STRATEGY,
			max_file_size: DEFAULT_MAX_FILE_SIZE,
			targets: DEFAULT_LOG_TARGETS.into(),
		}
	}

	pub fn rotation_strategy(mut self, rotation_strategy: RotationStrategy) -> Self {
		self.rotation_strategy = rotation_strategy;
		self
	}

	pub fn max_file_size(mut self, max_file_size: u128) -> Self {
		self.max_file_size = max_file_size;
		self
	}

	pub fn format<F>(mut self, formatter: F) -> Self
	where
		F: Fn(FormatCallback, &Arguments, &Record) + Send + Sync + 'static,
	{
		self.dispatch = self.dispatch.format(formatter);
		self
	}

	pub fn level(mut self, level_filter: impl Into<LevelFilter>) -> Self {
		self.dispatch = self.dispatch.level(level_filter.into());
		self
	}

	pub fn level_for(mut self, module: impl Into<Cow<'static, str>>, level: LevelFilter) -> Self {
		self.dispatch = self.dispatch.level_for(module, level);
		self
	}

	pub fn filter<F>(mut self, filter: F) -> Self
	where
		F: Fn(&log::Metadata) -> bool + Send + Sync + 'static,
	{
		self.dispatch = self.dispatch.filter(filter);
		self
	}

	pub fn target(mut self, target: LogTarget) -> Self {
		self.targets.push(target);
		self
	}

	pub fn targets(mut self, targets: impl IntoIterator<Item = LogTarget>) -> Self {
		self.targets = Vec::from_iter(targets);
		self
	}

	pub fn with_colors(self, colors: fern::colors::ColoredLevelConfig) -> Self {
		self.format(move |out, message, record| {
			out.finish(format_args!(
				"{}[{}][{}] {}",
				chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
				record.target(),
				colors.color(record.level()),
				message
			))
		})
	}

	pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
		plugin::Builder::new("log")
			.invoke_handler(tauri::generate_handler![log])
			.setup(move |app_handle| {
				let app_name = app_handle.package_info().name.as_str();

				for target in &self.targets {
					self.dispatch = self.dispatch.chain(match target {
						LogTarget::Stdout => fern::Output::from(std::io::stdout()),
						LogTarget::Stderr => fern::Output::from(std::io::stderr()),
						LogTarget::Folder(path) => {
							if !path.exists() {
								fs::create_dir_all(&path)?;
							}

							fern::log_file(get_log_file_path(
								&path,
								app_name,
								self.rotation_strategy,
								self.max_file_size,
							)?)?
							.into()
						}
						LogTarget::LogDir => {
							let path = app_handle
								.path_resolver()
								.log_dir()
								.expect("couldn't get log path");
							if !path.exists() {
								fs::create_dir_all(&path)?;
							}

							fern::log_file(get_log_file_path(
								&path,
								app_name,
								self.rotation_strategy,
								self.max_file_size,
							)?)?
							.into()
						}
						LogTarget::Webview => {
							let app_handle = app_handle.clone();

							fern::Output::call(move |record| {
								let payload = RecordPayload {
									message: record.args().to_string(),
									level: record.level().into(),
								};

								let app_handle = app_handle.clone();

								tauri::async_runtime::spawn(async move {
									app_handle.emit_all("log://log", payload).unwrap()
								});
							})
						}
					});
				}

				self.dispatch.apply()?;

				Ok(())
			})
			.build()
	}
}

impl Default for LoggerBuilder {
	fn default() -> Self {
		Self::new()
	}
}

fn get_log_file_path(
	dir: &Path,
	app_name: &str,
	rotation_strategy: RotationStrategy,
	max_file_size: u128,
) -> plugin::Result<PathBuf> {
	let path = dir.join(format!("{}.log", app_name));

	if path.exists() {
		let log_size = File::open(path.as_path())?.metadata()?.len() as u128;
		if log_size > max_file_size {
			match rotation_strategy {
				RotationStrategy::KeepAll => {
					let to = dir.join(format!(
						"{}_{}.log",
						app_name,
						chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
					));

					if to.is_file() {
						let mut to_bak = to.clone();
						to_bak.set_file_name(format!(
							"{}.bak",
							to_bak.file_name().unwrap().to_string_lossy()
						));
						fs::rename(&to, to_bak)?;
					}
					fs::rename(&path, to)?;
				}
				RotationStrategy::KeepOne => {
					fs::remove_file(&path)?;
				}
			}
		}
	}

	Ok(path)
}
