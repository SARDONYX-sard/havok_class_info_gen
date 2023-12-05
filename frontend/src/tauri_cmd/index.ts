import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { appLogDir } from '@tauri-apps/api/path';
import { open as openShell } from '@tauri-apps/api/shell';

import { selectLogLevel } from '@/utils/selector';

import tauriConfig from '@/../../src-tauri/tauri.conf.json';

type ExecuteArgs = {
  num1: string;
  num2: string;
};

/**
 * Convert string to number
 * # Throw Error
 */
function strToNum(str: string) {
  if (str === '' || Number.isNaN(Number(str))) {
    throw new Error('Invalid number passed');
  } else {
    return Number(str);
  }
}

/**
 * add two numbers
 * # Throw Error
 */
export async function add_num(props: ExecuteArgs): Promise<string> {
  let logLevel = selectLogLevel(localStorage.getItem('logLevel') ?? '');
  changeLogLevel(logLevel);

  const args = {
    num1: strToNum(props.num1),
    num2: strToNum(props.num2),
  };
  return invoke<string>('add_num', args);
}

export type LogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error';
export async function changeLogLevel(logLevel?: LogLevel): Promise<void> {
  return invoke('change_log_level', { logLevel });
}

/**
 * Open a file or Dir
 *
 * # Throw Error
 */
export async function openPath(path: string, setPath: (path: string) => void, isDir: boolean) {
  const res = await open({
    defaultPath: path,
    directory: isDir,
  });

  if (typeof res === 'string') {
    //! NOTE:
    //! It is important to use setter here!
    //! If we don't get the result within this function, somehow the previous value comes in.
    setPath(res);
  }
}

export async function openLogFile() {
  const logDir = await appLogDir();
  await openShell(`${logDir}${tauriConfig.package.productName}.log`);
}
