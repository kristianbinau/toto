import { reactive, ref, watch, computed } from "vue";
import { Store } from "@tauri-apps/plugin-store";
import {
  DEFAULT_STATE,
  SIMPLE_SCRIPT_ID,
  buildSimpleScript,
  type AppMode,
  type PersistedState,
  type ProfileEntry,
  type Script,
  type SimpleConfig,
} from "../lib/types";
import { totoApi } from "../lib/tauri";
import { reconcile, unbindAll, type HotkeyHandler } from "../lib/hotkeys";
import { errorToast } from "../lib/toast";

const STORE_FILE = "toto.json";
const STATE_KEY = "state";

type ArmEntry = { startedAt: number; delayMs: number; timerId: number };

type StoreShape = {
  simpleConfig: SimpleConfig;
  activeMode: AppMode;
  profiles: ProfileEntry[];
  runningIds: Set<string>;
  armingIds: Map<string, ArmEntry>;
  armTick: number;
  hydrated: boolean;
  hydrate: () => Promise<void>;
  dispose: () => Promise<void>;

  setActiveMode: (mode: AppMode) => void;
  updateSimpleConfig: (patch: Partial<SimpleConfig>) => void;
  simpleScript: () => Script;

  addProfile: () => ProfileEntry;
  updateProfile: (index: number, entry: ProfileEntry) => void;
  duplicateProfile: (index: number) => void;
  deleteProfile: (index: number) => Promise<void>;

  toggleScript: (script: Script) => Promise<void>;
  armOrCancel: (script: Script, delayMs: number) => void;
  cancelArm: (id: string) => void;
  isArming: (id: string) => boolean;
  armRemainingMs: (id: string) => number;
  stopAll: () => Promise<void>;

  isRunning: (id: string) => boolean;
};

let _singleton: StoreShape | null = null;
let _tauriStore: Store | null = null;
let _pollTimer: number | null = null;
let _saveTimer: number | null = null;
let _armTickTimer: number | null = null;
let _lastHotkeyReq = 0;
let _lastTrayActive: boolean | null = null;

function newProfile(index: number): ProfileEntry {
  return {
    script: {
      id: `profile-${index + 1}`,
      actions: [
        { type: "Click", button: "Left", direction: "Click" },
        { type: "Delay", ms: 100 },
      ],
      repeat: { mode: "Infinite" },
    },
  };
}

function deepClone<T>(v: T): T {
  return JSON.parse(JSON.stringify(v));
}

export function useProfilesStore(): StoreShape {
  if (_singleton) return _singleton;

  const simpleConfig = reactive<SimpleConfig>({ ...DEFAULT_STATE.simpleConfig });
  const profiles = reactive<ProfileEntry[]>([]);
  const activeMode = ref<AppMode>(DEFAULT_STATE.activeMode);
  const runningIds = reactive<Set<string>>(new Set());
  const armingIds = reactive<Map<string, ArmEntry>>(new Map());
  const armTick = ref(0);
  const hydrated = ref(false);

  const simpleScript = () => buildSimpleScript(simpleConfig);

  function schedulePersist() {
    if (!hydrated.value) return;
    if (_saveTimer !== null) clearTimeout(_saveTimer);
    _saveTimer = window.setTimeout(() => {
      _saveTimer = null;
      void persist();
    }, 250);
  }

  async function persist() {
    if (!_tauriStore) return;
    const snapshot: PersistedState = {
      version: 1,
      activeMode: activeMode.value,
      simpleConfig: deepClone(simpleConfig),
      profiles: deepClone(profiles),
    };
    try {
      await _tauriStore.set(STATE_KEY, snapshot);
      await _tauriStore.save();
    } catch (err) {
      errorToast("Failed to save settings", err);
    }
  }

  async function syncHotkeys() {
    const req = ++_lastHotkeyReq;
    const desired = new Map<string, HotkeyHandler>();
    if (simpleConfig.hotkey) {
      const accel = simpleConfig.hotkey;
      desired.set(accel, () => {
        armOrCancel(simpleScript(), 0);
      });
    }
    for (const entry of profiles) {
      if (!entry.hotkey) continue;
      const hotkey = entry.hotkey;
      desired.set(hotkey, () => {
        const current = profiles.find((p) => p.hotkey === hotkey);
        if (!current) return;
        armOrCancel(deepClone(current.script), 0);
      });
    }
    try {
      await reconcile(desired);
    } catch (err) {
      if (req === _lastHotkeyReq) {
        errorToast("Hotkey registration failed", err);
      }
    }
  }

  function cancelArm(id: string) {
    const entry = armingIds.get(id);
    if (!entry) return;
    clearTimeout(entry.timerId);
    armingIds.delete(id);
  }

  function armOrCancel(script: Script, delayMs: number) {
    const id = script.id;
    if (runningIds.has(id)) {
      void shape.toggleScript(script);
      return;
    }
    if (armingIds.has(id)) {
      cancelArm(id);
      return;
    }
    const scriptCopy = deepClone(script);
    const timerId = window.setTimeout(
      () => {
        armingIds.delete(id);
        void shape.toggleScript(scriptCopy);
      },
      Math.max(0, delayMs),
    );
    armingIds.set(id, { startedAt: Date.now(), delayMs, timerId });
  }

  function isArming(id: string): boolean {
    return armingIds.has(id);
  }

  function armRemainingMs(id: string): number {
    const e = armingIds.get(id);
    if (!e) return 0;
    return Math.max(0, e.delayMs - (Date.now() - e.startedAt));
  }

  function cancelAllArming() {
    for (const id of Array.from(armingIds.keys())) cancelArm(id);
  }

  async function refreshRunning() {
    try {
      const ids = await totoApi.runningScripts();
      runningIds.clear();
      for (const id of ids) runningIds.add(id);
    } catch (err) {
      errorToast("Failed to read running scripts", err);
      return;
    }
    const active = runningIds.size > 0;
    if (active !== _lastTrayActive) {
      _lastTrayActive = active;
      void totoApi.setTrayActive(active).catch(() => {});
    }
  }

  const shape: StoreShape = {
    simpleConfig,
    get activeMode(): AppMode {
      return activeMode.value;
    },
    set activeMode(v: AppMode) {
      activeMode.value = v;
    },
    profiles,
    runningIds,
    armingIds,
    get armTick(): number {
      return armTick.value;
    },
    get hydrated(): boolean {
      return hydrated.value;
    },

    async hydrate() {
      if (hydrated.value) return;
      _tauriStore = await Store.load(STORE_FILE, { autoSave: false, defaults: {} });
      const loaded = (await _tauriStore.get<PersistedState>(STATE_KEY)) ?? null;
      const initial = loaded ?? deepClone(DEFAULT_STATE);

      Object.assign(simpleConfig, initial.simpleConfig);
      profiles.splice(0, profiles.length, ...deepClone(initial.profiles));
      activeMode.value = initial.activeMode;
      hydrated.value = true;

      watch(simpleConfig, schedulePersist, { deep: true });
      watch(profiles, schedulePersist, { deep: true });
      watch(activeMode, schedulePersist);

      watch(
        [simpleConfig, profiles],
        () => {
          void syncHotkeys();
        },
        { deep: true },
      );

      await syncHotkeys();
      await refreshRunning();
      if (_pollTimer === null) {
        _pollTimer = window.setInterval(refreshRunning, 500);
      }
      if (_armTickTimer === null) {
        _armTickTimer = window.setInterval(() => {
          if (armingIds.size > 0) armTick.value++;
        }, 100);
      }
    },

    async dispose() {
      if (_pollTimer !== null) {
        clearInterval(_pollTimer);
        _pollTimer = null;
      }
      if (_armTickTimer !== null) {
        clearInterval(_armTickTimer);
        _armTickTimer = null;
      }
      cancelAllArming();
      await unbindAll();
    },

    setActiveMode(mode: AppMode) {
      activeMode.value = mode;
    },

    updateSimpleConfig(patch: Partial<SimpleConfig>) {
      Object.assign(simpleConfig, patch);
    },

    simpleScript,

    addProfile() {
      const entry = newProfile(profiles.length);
      profiles.push(entry);
      return entry;
    },

    updateProfile(index: number, entry: ProfileEntry) {
      const prev = profiles[index];
      if (!prev) return;
      // If the id changed, stop the old id first.
      if (prev.script.id !== entry.script.id) {
        cancelArm(prev.script.id);
        void totoApi.stopScript(prev.script.id).catch(() => {});
      }
      profiles.splice(index, 1, deepClone(entry));
    },

    duplicateProfile(index: number) {
      const src = profiles[index];
      if (!src) return;
      const copy = deepClone(src);
      copy.script.id = `${copy.script.id}-copy`;
      copy.hotkey = undefined;
      profiles.splice(index + 1, 0, copy);
    },

    async deleteProfile(index: number) {
      const entry = profiles[index];
      if (!entry) return;
      cancelArm(entry.script.id);
      await totoApi.stopScript(entry.script.id).catch(() => {});
      profiles.splice(index, 1);
    },

    async toggleScript(script: Script) {
      try {
        await totoApi.toggleScript(script);
      } catch (err) {
        errorToast("Failed to toggle script", err);
      }
      await refreshRunning();
    },

    armOrCancel,
    cancelArm,
    isArming,
    armRemainingMs,

    async stopAll() {
      cancelAllArming();
      try {
        await totoApi.stopAll();
      } catch (err) {
        errorToast("Failed to stop scripts", err);
      }
      await refreshRunning();
    },

    isRunning(id: string) {
      return runningIds.has(id);
    },
  };

  _singleton = shape;
  return shape;
}

export function useRunningCount() {
  const store = useProfilesStore();
  return computed(() => store.runningIds.size);
}

export { SIMPLE_SCRIPT_ID };
