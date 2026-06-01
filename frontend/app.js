const { invoke } = window.__TAURI__.core;

export async function invokeCommand(cmd, args = {}) {
  try {
    return await invoke(cmd, args);
  } catch (e) {
    throw new Error(e);
  }
}
