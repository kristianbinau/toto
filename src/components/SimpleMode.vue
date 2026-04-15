<script setup lang="ts">
import { computed } from "vue";
import { useProfilesStore } from "../stores/profiles";
import { SIMPLE_SCRIPT_ID } from "../lib/types";
import HotkeyInput from "./common/HotkeyInput.vue";

const store = useProfilesStore();
const running = computed(() => store.runningIds.has(SIMPLE_SCRIPT_ID));

const buttonItems = [
  { label: "Left", value: "Left" as const },
  { label: "Right", value: "Right" as const },
];

function setButton(v: "Left" | "Right") {
  store.simpleConfig.button = v;
}

function setInterval(v: number) {
  const n = Number(v);
  if (!Number.isFinite(n)) return;
  store.simpleConfig.intervalMs = Math.max(10, Math.min(60000, Math.floor(n)));
}

function setHotkey(v: string | undefined) {
  store.simpleConfig.hotkey = v;
}

async function toggle() {
  await store.toggleScript(store.simpleScript());
}
</script>

<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <h2 class="text-sm font-semibold">Simple autoclicker</h2>
        <UBadge
          :color="running ? 'success' : 'neutral'"
          :variant="running ? 'solid' : 'subtle'"
          size="sm"
        >
          {{ running ? "Running" : "Idle" }}
        </UBadge>
      </div>
    </template>

    <div class="flex flex-col gap-3">
      <div>
        <label class="text-xs text-neutral-500 block mb-1">Button</label>
        <div class="flex gap-2">
          <UButton
            v-for="item in buttonItems"
            :key="item.value"
            size="sm"
            :color="store.simpleConfig.button === item.value ? 'primary' : 'neutral'"
            :variant="store.simpleConfig.button === item.value ? 'solid' : 'outline'"
            block
            @click="setButton(item.value)"
          >
            {{ item.label }}
          </UButton>
        </div>
      </div>

      <div>
        <label class="text-xs text-neutral-500 block mb-1">Interval (ms)</label>
        <UInput
          type="number"
          :min="10"
          :max="60000"
          :model-value="store.simpleConfig.intervalMs"
          @update:model-value="setInterval($event as number)"
          size="sm"
        />
      </div>

      <div>
        <label class="text-xs text-neutral-500 block mb-1">Toggle hotkey</label>
        <HotkeyInput
          :model-value="store.simpleConfig.hotkey"
          @update:model-value="setHotkey"
        />
      </div>

      <UButton
        :color="running ? 'error' : 'primary'"
        :icon="running ? 'i-lucide-square' : 'i-lucide-play'"
        size="md"
        block
        @click="toggle"
      >
        {{ running ? "Stop" : "Start" }}
      </UButton>
    </div>
  </UCard>
</template>
