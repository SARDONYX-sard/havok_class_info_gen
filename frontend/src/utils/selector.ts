import { LogLevel } from '@/tauri_cmd';

export function selectLogLevel(logLevel: string): LogLevel {
  switch (logLevel) {
    case 'trace':
    case 'debug':
    case 'info':
    case 'warn':
    case 'error':
      return logLevel;
    default:
      return 'error';
  }
}
