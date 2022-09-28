import { invoke } from '@tauri-apps/api/tauri';
import { Event, listen } from '@tauri-apps/api/event';

enum LogLevel {
	Trace = 1,
	Debug,
	Info,
	Warn,
	Error,
}

async function log(level: LogLevel, message: string): Promise<void> {
	const traces = new Error().stack?.split('\n').map((line) => line.split('@'));

	const filtered = traces?.filter(([name, location]) => {
		return name.length && location !== '[native code]';
	});

	await invoke('plugin:log|log', { level, message, location: filtered?.[0]?.join('@') });
}

export async function error(message: string): Promise<void> {
	await log(LogLevel.Error, message);
}

export async function warn(message: string): Promise<void> {
	await log(LogLevel.Warn, message);
}

export async function info(message: string): Promise<void> {
	await log(LogLevel.Info, message);
}

export async function debug(message: string): Promise<void> {
	await log(LogLevel.Debug, message);
}

export async function trace(message: string): Promise<void> {
	await log(LogLevel.Trace, message);
}

interface RecordPayload {
	level: LogLevel;
	message: string;
}

export function attachConsole() {
	return listen('log://log', (event: Event<RecordPayload>) => {
		const payload = event.payload;

		switch (payload.level) {
			case LogLevel.Trace:
				console.log(payload.message);
				break;
			case LogLevel.Debug:
				console.debug(payload.message);
				break;
			case LogLevel.Info:
				console.info(payload.message);
				break;
			case LogLevel.Warn:
				console.warn(payload.message);
				break;
			case LogLevel.Error:
				console.error(payload.message);
				break;
			default:
				throw new Error(`unknown log level ${payload.level}`);
		}
	});
}
