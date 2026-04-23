import { register, unregister, unregisterAll } from "@tauri-apps/plugin-global-shortcut";

export type HotkeyHandler = () => void;

const active = new Map<string, HotkeyHandler>();

export async function bindHotkey(accel: string, handler: HotkeyHandler): Promise<void> {
  await unregister(accel).catch(() => {});
  active.delete(accel);
  await register(accel, (event) => {
    if (event.state === "Pressed") handler();
  });
  active.set(accel, handler);
}

export async function unbindHotkey(accel: string): Promise<void> {
  if (!active.has(accel)) return;
  await unregister(accel).catch(() => {});
  active.delete(accel);
}

export async function unbindAll(): Promise<void> {
  await unregisterAll().catch(() => {});
  active.clear();
}

/** Reconcile the set of registered accelerators with a desired map. */
export async function reconcile(desired: Map<string, HotkeyHandler>): Promise<void> {
  for (const accel of Array.from(active.keys())) {
    if (!desired.has(accel)) await unbindHotkey(accel);
  }
  for (const [accel, handler] of desired) {
    if (!active.has(accel)) await bindHotkey(accel, handler);
  }
}

/** Format a DOM KeyboardEvent into a Tauri-format accelerator string. */
export function formatKeyEvent(e: KeyboardEvent): string | null {
  const parts: string[] = [];
  if (e.ctrlKey || e.metaKey) parts.push("CmdOrCtrl");
  if (e.altKey) parts.push("Alt");
  if (e.shiftKey) parts.push("Shift");

  const k = e.key;
  // Ignore bare modifier presses.
  if (k === "Control" || k === "Shift" || k === "Alt" || k === "Meta") {
    return null;
  }

  let keyPart: string;
  if (k.length === 1) {
    keyPart = k.toUpperCase();
  } else if (/^F\d+$/.test(k)) {
    keyPart = k;
  } else if (k === "ArrowUp") keyPart = "Up";
  else if (k === "ArrowDown") keyPart = "Down";
  else if (k === "ArrowLeft") keyPart = "Left";
  else if (k === "ArrowRight") keyPart = "Right";
  else if (k === " ") keyPart = "Space";
  else keyPart = k;

  parts.push(keyPart);
  return parts.join("+");
}
