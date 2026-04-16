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

Replace `App.vue` with a real layout. Use NuxtUI components throughout (`UButton`, `UInput`, `USelect`, `UCard`, `UBadge`, `UKbd`, `UDropdownMenu`, etc.). Two modes: **Simple** (one-button autoclicker) and **Advanced** (full profile editor matching the engine capability surface).

- [x] Typed invoke wrapper ([src/lib/tauri.ts](src/lib/tauri.ts)) + TS mirror of engine types ([src/lib/types.ts](src/lib/types.ts))
- [x] Profiles store composable ([src/stores/profiles.ts](src/stores/profiles.ts)) — single source of truth for UI state, wraps `totoApi`
- [x] App shell with Simple/Advanced mode switch ([src/App.vue](src/App.vue))
- [x] Simple mode: Left/Right button picker, interval input, start/stop ([src/components/SimpleMode.vue](src/components/SimpleMode.vue)) — builds a preset `Script` with id `__simple__` and drives it through `toggle_script`
- [x] Advanced mode: profile list, profile editor, action list editor ([src/components/advanced/](src/components/advanced/))
- [x] `ActionRow` component covering Click / Key / Move / Delay ([src/components/advanced/ActionRow.vue](src/components/advanced/ActionRow.vue))
- [x] Running-state poll (500ms) wired to `running_scripts` command — handled inside the profiles store, reconciles `runningIds` so indicators clear when a `Times(n)` script finishes naturally
- [x] Stop-all destructive button in the profile list

## Phase 3 — Persistence & Profiles

Persist UI state via `@tauri-apps/plugin-store`; bind per-profile and simple-mode toggle hotkeys via `@tauri-apps/plugin-global-shortcut`. Everything centralised in [src/stores/profiles.ts](src/stores/profiles.ts).

- [x] Hydrate store from `toto.json` on startup via `Store.load` — runs in [src/main.ts](src/main.ts) before `app.mount`
- [x] Debounced auto-save (~250ms) on any state change via `watch(..., { deep: true })`
- [x] First-run seed: default simple config (`Left`, 100 ms) and empty profile list
- [x] `HotkeyInput` component ([src/components/common/HotkeyInput.vue](src/components/common/HotkeyInput.vue)) — captures a keydown and normalises to a Tauri accelerator string (`CmdOrCtrl+Shift+K`)
- [x] Register/unregister per-profile and simple-mode hotkeys via plugin-global-shortcut; centralised reconciliation in [src/lib/hotkeys.ts](src/lib/hotkeys.ts)
- [x] Profile delete / rename: stops the old script id and re-reconciles hotkeys automatically

## Phase 4 — System Tray

- [x] Add tray icon using Tauri 2 core (`tray-icon` + `image-png` features in `Cargo.toml`; no separate plugin)
- [x] Tray menu items: "Show", "About", "Quit" (Start/Stop intentionally omitted — running state is managed from the window)
- [x] Tray icon changes when clicking is active (`tray-idle-32.png` / `tray-active-32.png`, swapped via `set_tray_active` command)
- [x] Closing the window hides it (minimise to tray); left-clicking the tray icon shows it again
  - `CloseRequested` handler calls `api.prevent_close()` then `window.hide()`

---

## Phase 5 — Polish

- [x] App icon regenerated from [logo.svg](logo.svg) via `npx @tauri-apps/cli icon logo.svg`
- [x] Keyboard-accessible UI (all controls use NuxtUI primitives; icon-only buttons have `aria-label`)
- [x] Error toasts for failed commands via `src/lib/toast.ts` + `useToast` sink wired in [src/App.vue](src/App.vue)
- [x] About dialog with version number (reads via `@tauri-apps/api/app::getVersion`); opens from both the header info button and the tray "About" menu item
- [x] Windows / Linux (X11 / Wayland) limitations documented in [README.md](README.md#platform-notes)

---

## Known Constraints & Notes

- **enigo 0.6.1 API**: The primary click method is `enigo.mouse_click(Button::Left, Click::Click)`. Check the docs at https://docs.rs/enigo/0.6.1/enigo/ — the API changed significantly from 0.5.x.
- **enigo thread safety**: `enigo::Enigo` is not `Send` on all platforms. Create a new `Enigo` instance inside the click thread rather than sharing one across threads.
- **Click interval floor**: Intervals below ~10ms are unreliable on most OSes due to scheduler granularity. Enforce a minimum of 10ms in the UI and clamp in the Rust command handler.
- **Wayland**: `enigo` uses `xdotool`-style X11 simulation; on Wayland it may require `XWayland`. Document this limitation.
- **tauri-plugin-store**: The store is async — all commands that read/write it must be `async fn` and use `.await`.
- **NuxtUI in Vue (non-Nuxt)**: Follow the Vue-specific installation exactly. The Nuxt auto-import magic is not available; import components explicitly or configure the Vite resolver as shown in the docs.
