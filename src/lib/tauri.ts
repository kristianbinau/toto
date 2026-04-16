import { invoke } from "@tauri-apps/api/core";
import type { Script } from "./types";

export const totoApi = {
  startScript: (script: Script) => invoke<void>("start_script", { script }),
  stopScript: (id: string) => invoke<void>("stop_script", { id }),
  toggleScript: (script: Script) => invoke<boolean>("toggle_script", { script }),
  isRunning: (id: string) => invoke<boolean>("is_running", { id }),
  runningScripts: () => invoke<string[]>("running_scripts"),
  stopAll: () => invoke<void>("stop_all"),
  setTrayActive: (active: boolean) => invoke<void>("set_tray_active", { active }),
};
