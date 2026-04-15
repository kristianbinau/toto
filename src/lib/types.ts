// TS mirrors of the engine types in crates/toto-engine/src/{types.rs,action/mod.rs,repeat.rs,script.rs}.
// Shapes MUST match serde tags exactly.

export type MouseButton = "Left" | "Right" | "Middle";
export type Direction = "Press" | "Release" | "Click";
export type Coord = "Absolute" | "Relative";

export type Key =
  | { name: "Unicode"; value: string }
  | { name: "Return" }
  | { name: "Tab" }
  | { name: "Space" }
  | { name: "Escape" }
  | { name: "Backspace" };

export type Action =
  | { type: "Click"; button: MouseButton; direction: Direction }
  | { type: "Key"; key: Key; direction: Direction }
  | { type: "Move"; x: number; y: number; coord: Coord }
  | { type: "Delay"; ms: number };

export type ActionKind = Action["type"];

export type Repeat =
  | { mode: "Once" }
  | { mode: "Times"; count: number }
  | { mode: "Infinite" };

export type Script = {
  id: string;
  actions: Action[];
  repeat: Repeat;
};

export type SimpleConfig = {
  button: "Left" | "Right";
  intervalMs: number;
  hotkey?: string;
};

export type ProfileEntry = {
  script: Script;
  hotkey?: string;
};

export type AppMode = "simple" | "advanced";

export type PersistedState = {
  version: 1;
  activeMode: AppMode;
  simpleConfig: SimpleConfig;
  profiles: ProfileEntry[];
};

export const SIMPLE_SCRIPT_ID = "__simple__";

export const DEFAULT_STATE: PersistedState = {
  version: 1,
  activeMode: "simple",
  simpleConfig: { button: "Left", intervalMs: 100 },
  profiles: [],
};

export function buildSimpleScript(cfg: SimpleConfig): Script {
  return {
    id: SIMPLE_SCRIPT_ID,
    repeat: { mode: "Infinite" },
    actions: [
      { type: "Click", button: cfg.button, direction: "Click" },
      { type: "Delay", ms: Math.max(0.1, cfg.intervalMs) },
    ],
  };
}

export function defaultActionFor(kind: ActionKind): Action {
  switch (kind) {
    case "Click":
      return { type: "Click", button: "Left", direction: "Click" };
    case "Key":
      return { type: "Key", key: { name: "Return" }, direction: "Click" };
    case "Move":
      return { type: "Move", x: 0, y: 0, coord: "Relative" };
    case "Delay":
      return { type: "Delay", ms: 100 };
  }
}

export function summarizeScript(s: Script): string {
  const n = s.actions.length;
  const r =
    s.repeat.mode === "Once"
      ? "Once"
      : s.repeat.mode === "Infinite"
        ? "Infinite"
        : `x${s.repeat.count}`;
  return `${n} action${n === 1 ? "" : "s"} · ${r}`;
}
