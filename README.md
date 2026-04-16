# toto — Minimalistic Autoclicker

A fast, lightweight autoclicker / macro runner built with **Tauri 2** (Rust backend) and **Vue 3** (frontend). Designed to be unobtrusive: small window, always-on-top, global hotkeys to toggle scripts without switching focus.

## Design Principle

**All logic lives in Rust.** The Vue frontend is purely a config editor / hotkey binder — no timing, no execution state, nothing that would need re-implementing if the UI were swapped. The engine is a standalone, reusable Rust library ([`crates/toto-engine/`](crates/toto-engine/)) with its own tests, so a future CLI or different UI can consume it without touching Tauri.

## Tech Stack

| Layer                | Technology                                                                                   |
| -------------------- | -------------------------------------------------------------------------------------------- |
| UI Framework         | Vue 3 + TypeScript                                                                           |
| Component Library    | [NuxtUI for Vue](https://ui.nuxt.com/docs/getting-started/installation/vue) (Tailwind-based) |
| Build Tool           | Vite 6                                                                                       |
| Desktop Shell        | Tauri 2                                                                                      |
| Input Simulation     | [enigo 0.6.1](https://docs.rs/enigo/0.6.1/enigo/)                                            |
| Settings Persistence | [tauri-plugin-store](https://v2.tauri.app/plugin/store/)                                     |
| Global Hotkeys       | [tauri-plugin-global-shortcut](https://v2.tauri.app/plugin/global-shortcut/)                 |

## Repository Layout

```
toto/
├── Cargo.toml                   # Cargo workspace root
├── crates/
│   └── toto-engine/             # Headless automation library (UI-agnostic)
│       ├── src/
│       │   ├── action/          # click, keypress, movement, delay — one module each
│       │   ├── backend/         # Backend trait + EnigoBackend (prod) + MockBackend (tests)
│       │   ├── engine.rs        # Top-level facade: start/stop/toggle/stop_all
│       │   ├── runner.rs        # Thread-per-script with stop-signal channel
│       │   ├── script.rs        # Script { id, actions, repeat }
│       │   ├── repeat.rs        # Once | Times(n) | Infinite
│       │   ├── exec.rs          # ExecCtx — backend + interruptible sleep
│       │   └── types.rs         # Shared primitives (MouseButton, Direction, Key, Coord)
│       └── tests/engine.rs      # Integration tests (concurrent scripts, prompt stop, …)
├── src-tauri/                   # Tauri shell — thin pass-through to toto-engine
│   ├── src/
│   │   ├── lib.rs               # Plugin registration + Engine state + command handlers
│   │   └── commands.rs          # #[tauri::command] wrappers
│   ├── capabilities/default.json
│   └── tauri.conf.json
├── src/                         # Vue 3 frontend (config editor)
│   ├── App.vue
│   ├── main.ts
│   └── assets/main.css          # Tailwind + NuxtUI imports
└── ROADMAP.md
```

## Development Setup

### Prerequisites

- [Node.js](https://nodejs.org/) 20.19+ (NuxtUI requires it)
- [Rust](https://rustup.rs/) stable toolchain
- Tauri system dependencies for your OS — see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
- Linux: `enigo` uses X11. On Wayland sessions, run under XWayland or set `WINIT_UNIX_BACKEND=x11`.

### First-time install

```bash
npm install
```

This installs the Vue toolchain, NuxtUI, `vue-router` (peer of NuxtUI), and the Tauri JS plugin bindings. Rust crates are fetched on first `cargo build`.

## Commands

All commands are run from the **repository root**.

### Run the app

```bash
npm run tauri dev      # Vite + Tauri with hot reload
npm run tauri build    # Production bundle (in target/release/bundle/)
npm run tauri info     # Report toolchain + plugin versions
```

### Frontend only

```bash
npm run dev            # Vite dev server (no Tauri window) — usually not used directly
npm run build          # Type-check (vue-tsc) + vite build — outputs to dist/
```

### Engine (Rust library)

The engine is a regular Cargo workspace member and can be built or tested without involving Tauri.

```bash
cargo test -p toto-engine --no-default-features   # headless unit + integration tests (uses MockBackend only)
cargo test -p toto-engine                          # also compiles the enigo backend
cargo build -p toto-engine                         # library build
cargo build --workspace                            # everything, including the Tauri shell
```

The `--no-default-features` flag disables the `enigo-backend` feature so the test suite runs without pulling X11/linker dependencies. Tests never touch real input — they use `MockBackend`, which records events into a shared buffer.

#### Sleep-precision benchmark

```bash
cargo bench -p toto-engine
```

Runs `benches/sleep_accuracy.rs` via Criterion and measures actual sleep duration at 0.1, 0.5, 1, 2, 5, 10, 20, and 50 ms. HTML reports are written to `target/criterion/`. Use this to verify the spin-wait and hybrid thresholds on your hardware — expected ranges on a typical Windows machine:

| Target  | Acceptable range | Strategy     |
| ------- | ---------------- | ------------ |
| 0.1 ms  | 0.05 – 0.5 ms    | spin         |
| 0.5 ms  | 0.4 – 1.0 ms     | spin         |
| 1.0 ms  | 0.9 – 1.5 ms     | spin         |
| 5.0 ms  | 4.5 – 6.5 ms     | hybrid       |
| 10.0 ms | 9.0 – 12.0 ms    | hybrid       |
| 50.0 ms | 48 – 53 ms       | recv_timeout |

Sub-2 ms delays consume one CPU core while spinning; this is intentional and expected.

### Tauri commands (from the frontend)

The backend exposes these commands via `@tauri-apps/api/core`'s `invoke(...)`. All payload types are `Serialize`/`Deserialize` on the engine types, so they cross the boundary directly.

| Command           | Args                 | Returns                   | Purpose                                                          |
| ----------------- | -------------------- | ------------------------- | ---------------------------------------------------------------- |
| `start_script`    | `{ script: Script }` | `Result<(), string>`      | Start a script. Errors if already running.                       |
| `stop_script`     | `{ id: string }`     | `Result<(), string>`      | Stop a running script by id.                                     |
| `toggle_script`   | `{ script: Script }` | `Result<boolean, string>` | Start if not running, stop if running. Returns new active state. |
| `is_running`      | `{ id: string }`     | `boolean`                 | Whether a script with that id is currently active.               |
| `running_scripts` | —                    | `string[]`                | Ids of all currently active scripts.                             |
| `stop_all`        | —                    | `void`                    | Stop every running script.                                       |

`Script` JSON shape:

```ts
type Script = {
  id: string;
  actions: Action[];
  repeat?: { mode: "Once" } | { mode: "Times"; count: number } | { mode: "Infinite" };
};

type Action =
  | { type: "Click"; button: "Left" | "Right" | "Middle"; direction: "Press" | "Release" | "Click" }
  | {
      type: "Key";
      key:
        | { name: "Unicode"; value: string }
        | { name: "Return" }
        | { name: "Tab" }
        | { name: "Space" }
        | { name: "Escape" }
        | { name: "Backspace" };
      direction: "Press" | "Release" | "Click";
    }
  | { type: "Move"; x: number; y: number; coord: "Absolute" | "Relative" }
  | { type: "Delay"; ms: number }; // fractional ms supported (e.g. 0.5 = 500 µs); minimum 0.1 enforced by the UI
```

Example from the devtools console:

```js
const script = {
  id: "demo",
  repeat: { mode: "Times", count: 5 },
  actions: [
    { type: "Click", button: "Left", direction: "Click" },
    { type: "Delay", ms: 200 },
  ],
};
await window.__TAURI__.core.invoke("start_script", { script });
```

## UI modes

The window is a fixed 340×480, always-on-top. Two modes, switchable from the header:

### Simple mode

Single-click configuration for the common case:

- Button: **Left** or **Right**
- Interval: milliseconds between clicks (fractional values supported, e.g. 0.5 ms; minimum 0.1 ms)
- Optional global toggle hotkey

Starting it synthesises a preset `Script` with id `__simple__`:

```ts
{
  id: "__simple__",
  repeat: { mode: "Infinite" },
  actions: [
    { type: "Click", button: "Left", direction: "Click" },
    { type: "Delay", ms: 100 },
  ],
}
```

It goes through the normal `toggle_script` command — Rust has no special case for simple mode.

### Advanced mode

A profile list with full CRUD. Each profile owns:

- A name that is used verbatim as the engine's `ScriptId`
- An ordered action list (`Click` / `Key` / `Move` / `Delay`) edited in place with reorder and delete
- A `Repeat` mode: `Once` / `Times(n)` / `Infinite`
- An optional toggle hotkey

The profile list has a destructive **Stop all** button wired to `stop_all`. Running state (the green badge and `Stop` labels) is driven by a 500 ms poll of `running_scripts`, so indicators clear even when a `Times(n)` script finishes on its own or a hotkey toggles it while the editor is closed.

## Persistence

UI state is persisted via `@tauri-apps/plugin-store` into a single file, `toto.json`, in the platform's `app_data_dir`. Schema:

```ts
type PersistedState = {
  version: 1;
  activeMode: "simple" | "advanced";
  simpleConfig: { button: "Left" | "Right"; intervalMs: number; hotkey?: string };
  profiles: Array<{
    script: Script; // same shape as the tauri command payload
    hotkey?: string; // e.g. "CmdOrCtrl+Shift+1"
  }>;
};
```

Writes are debounced (~250 ms) and triggered from a deep `watch` on the reactive store. The profiles composable is the single source of truth; components never touch the store plugin directly.

Global hotkeys are bound via `@tauri-apps/plugin-global-shortcut`. When the state changes, [src/lib/hotkeys.ts](src/lib/hotkeys.ts) reconciles the set of registered accelerators (unregister removed, register added). Each bound hotkey simply invokes `toggle_script` with the associated profile's script.

## Architecture Notes

### Concurrency model

- Each running script owns its own OS thread.
- Each thread constructs its own `EnigoBackend` inside the thread (enigo is not `Send` on all platforms — see [engine `BackendFactory`](crates/toto-engine/src/runner.rs)).
- Stop is signalled through an `mpsc::channel::<()>`. `Delay::execute` dispatches to one of three sleep strategies based on duration: pure spin-wait (≤ 2 ms, for sub-millisecond precision), a hybrid coarse-sleep + spin-tail (2–20 ms, to absorb OS scheduler overshoot), or plain `recv_timeout` (> 20 ms, CPU-friendly). In all cases the stop signal is checked throughout, so scripts halt promptly.
- Multiple scripts run fully independently — starting, stopping, or toggling one script never affects the others. This is what enables "different hotkey, different active config, all running simultaneously".

### Adding a new action type

1. Create a new file under [crates/toto-engine/src/action/](crates/toto-engine/src/action/) with a `struct` and an `execute(&self, ctx: &mut ExecCtx<'_>) -> ActionResult` method.
2. Add a variant to the `Action` enum in [action/mod.rs](crates/toto-engine/src/action/mod.rs) and a match arm in `Action::execute`.
3. Add any new side-effect method to the [`Backend`](crates/toto-engine/src/backend/mod.rs) trait, with implementations in both `EnigoBackend` and `MockBackend`.
4. Add a unit test alongside the new module. Nothing in `src-tauri/` needs to change — `Script` serialisation picks up the new variant automatically.

### Settings persistence

Profiles, the simple-mode config, the active mode, and hotkey bindings are persisted via `tauri-plugin-store` — see the [Persistence](#persistence) section above for the schema. The profiles composable in [src/stores/profiles.ts](src/stores/profiles.ts) owns hydration, debounced save, and hotkey reconciliation.

## Platform notes

### Windows

`enigo` cannot deliver synthetic input to windows running at a higher integrity level than toto itself. In practice this means: if a UAC-elevated window is focused, clicks from toto are silently dropped. To drive an elevated target, launch toto elevated too. Running as a standard user is otherwise fine — no UAC prompt is required at startup.

### Linux

- **X11** (including XWayland): `enigo` works out of the box via the XDO backend.
- **Wayland**: most compositors refuse synthetic input from arbitrary apps for security reasons. Running under XWayland fixes this for apps that accept X11 input. If toto itself is launched on a native Wayland session, export `WINIT_UNIX_BACKEND=x11` before `npm run tauri dev` / the built binary to force the X11 winit backend.

## Roadmap

See [ROADMAP.md](ROADMAP.md). Phases 0–5 are complete.
