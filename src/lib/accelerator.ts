// Parse Tauri accelerator strings into tokens for <UKbd> rendering.
// Example: "CmdOrCtrl+Shift+A" -> [{value:"meta"}, {value:"shift"}, {literal:"A"}]

export type KbdToken = { value: string } | { literal: string };

const MODIFIERS: Record<string, string> = {
  CmdOrCtrl: "meta",
  Cmd: "meta",
  Command: "meta",
  Ctrl: "meta",
  Control: "meta",
  Meta: "meta",
  Super: "meta",
  Alt: "alt",
  Option: "alt",
  Shift: "shift",
};

const NAMED_KEYS: Record<string, string> = {
  Space: "space",
  Enter: "enter",
  Return: "enter",
  Tab: "tab",
  Escape: "esc",
  Esc: "esc",
  Backspace: "backspace",
  Delete: "delete",
  Up: "arrowup",
  Down: "arrowdown",
  Left: "arrowleft",
  Right: "arrowright",
};

export function parseAccelerator(accel: string): KbdToken[] {
  if (!accel) return [];
  const parts = accel
    .split("+")
    .map((p) => p.trim())
    .filter(Boolean);
  return parts.map((part) => {
    if (MODIFIERS[part]) return { value: MODIFIERS[part] };
    if (NAMED_KEYS[part]) return { value: NAMED_KEYS[part] };
    return { literal: part };
  });
}
