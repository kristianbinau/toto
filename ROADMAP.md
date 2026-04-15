# ROADMAP

Implementation order is top-to-bottom. Each phase must be fully working before the next begins. Check items off as they are completed.

---

## Phase 0 — Dependencies & Scaffolding

Set up all third-party dependencies before writing any feature code.

### Frontend

- [x] Install NuxtUI for Vue following https://ui.nuxt.com/docs/getting-started/installation/vue
  - `npm install @nuxt/ui`
  - Add the Vite plugin and Tailwind CSS config as instructed
  - Add the NuxtUI plugin to `src/main.ts`
  - Verify a basic `<UButton>` renders in `App.vue`
- [x] Install `@tauri-apps/plugin-store` JS bindings: `npm install @tauri-apps/plugin-store`
- [x] Install `@tauri-apps/plugin-global-shortcut` JS bindings: `npm install @tauri-apps/plugin-global-shortcut`

### Rust / Tauri

- [x] Add `enigo` to `Cargo.toml`:
  ```toml
  enigo = "0.6.1"
  ```
- [x] Add `tauri-plugin-store` to `Cargo.toml` and register it in `lib.rs`:
  ```toml
  tauri-plugin-store = "2"
  ```
  See https://v2.tauri.app/plugin/store/
- [x] Add `tauri-plugin-global-shortcut` to `Cargo.toml` and register it in `lib.rs`:
  ```toml
  tauri-plugin-global-shortcut = "2"
  ```
  See https://v2.tauri.app/plugin/global-shortcut/

### Tauri Config

- [x] Update `tauri.conf.json` window settings:
  ```json
  {
    "width": 340,
    "height": 480,
    "resizable": false,
    "alwaysOnTop": true,
    "decorations": false
  }
  ```
- [x] Add required plugin permissions to `capabilities/default.json` for store and global-shortcut

---

## Phase 1 — Rust Clicking Engine

Build the backend before the UI. Everything should be testable via Tauri devtools and tested via Rust unit tests.

- [x] Workspace with standalone `toto-engine` crate under [crates/toto-engine/](crates/toto-engine/) — UI-agnostic, reusable
- [x] Action modules: [click.rs](crates/toto-engine/src/action/click.rs), [keypress.rs](crates/toto-engine/src/action/keypress.rs), [movement.rs](crates/toto-engine/src/action/movement.rs), [delay.rs](crates/toto-engine/src/action/delay.rs)
- [x] `Backend` trait with `EnigoBackend` (production) and `MockBackend` (tests)
- [x] `ScriptRunner` — thread-per-script with stop-aware sleep for prompt interruption
- [x] `Engine` — multiple concurrent scripts, start/stop/toggle/is_running/stop_all
- [x] Unit tests per action + integration tests covering concurrent scripts, prompt stop, toggle, JSON round-trip
- [x] Tauri commands in [src-tauri/src/commands.rs](src-tauri/src/commands.rs) as thin pass-throughs

## Phase 2 — Vue UI

Replace `App.vue` with a real layout. Use NuxtUI components throughout (`UButton`, `UInput`, `USelect`, `UCard`, `UToggle`, `UBadge`, etc.).

## Phase 3 — Persistence & Profiles

## Phase 4 — System Tray

- [ ] Add tray icon using `tauri-plugin-tray` (built into Tauri 2 core, enable in `Cargo.toml` features)
- [ ] Tray menu items: "Show", "Start/Stop", "Quit"
- [ ] Tray icon changes when clicking is active (different icon file)
- [ ] Closing the window hides it (minimise to tray); clicking the tray icon shows it again
  - Handle the `CloseRequested` window event, call `event.prevent_close()`, then `window.hide()`

---

## Phase 5 — Polish

- [ ] App icon (replace default Tauri icons in `src-tauri/icons/`)
- [ ] Keyboard-accessible UI (all inputs reachable via Tab)
- [ ] Error toasts for failed commands (use NuxtUI `useToast`)
- [ ] About dialog with version number (read from `tauri.conf.json` via `app.package_info()`)
- [ ] Windows: test that `enigo` simulation works without UAC issues on standard user accounts
- [ ] Linux: test under X11 and Wayland (enigo has different backends; may need `WINIT_UNIX_BACKEND=x11`)

---

## Known Constraints & Notes

- **enigo 0.6.1 API**: The primary click method is `enigo.mouse_click(Button::Left, Click::Click)`. Check the docs at https://docs.rs/enigo/0.6.1/enigo/ — the API changed significantly from 0.5.x.
- **enigo thread safety**: `enigo::Enigo` is not `Send` on all platforms. Create a new `Enigo` instance inside the click thread rather than sharing one across threads.
- **Click interval floor**: Intervals below ~10ms are unreliable on most OSes due to scheduler granularity. Enforce a minimum of 10ms in the UI and clamp in the Rust command handler.
- **Wayland**: `enigo` uses `xdotool`-style X11 simulation; on Wayland it may require `XWayland`. Document this limitation.
- **tauri-plugin-store**: The store is async — all commands that read/write it must be `async fn` and use `.await`.
- **NuxtUI in Vue (non-Nuxt)**: Follow the Vue-specific installation exactly. The Nuxt auto-import magic is not available; import components explicitly or configure the Vite resolver as shown in the docs.
