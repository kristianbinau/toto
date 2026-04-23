<script setup lang="ts">
import { computed } from "vue";
import { useProfilesStore } from "../stores/profiles";
import HotkeyInput from "./common/HotkeyInput.vue";

defineEmits<{ back: [] }>();

const store = useProfilesStore();

const buttonItems = [
  { label: "Left", value: "Left" },
  { label: "Right", value: "Right" },
];

const intervalMs = computed({
  get: () => store.simpleConfig.intervalMs,
  set: (v: number) => {
    const n = Number(v);
    if (!Number.isFinite(n)) return;
    store.simpleConfig.intervalMs = Math.max(0.1, Math.min(60_000, n));
  },
});

const button = computed({
  get: () => store.simpleConfig.button,
  set: (v: "Left" | "Right") => {
    store.simpleConfig.button = v;
  },
});

const hotkey = computed({
  get: () => store.simpleConfig.hotkey,
  set: (v: string | undefined) => {
    store.simpleConfig.hotkey = v;
  },
});
</script>

<template>
  <UCard :ui="{ body: 'p-3 sm:p-3' }">
    <template #header>
      <div class="flex items-center gap-2">
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-arrow-left"
          aria-label="Back"
          @click="$emit('back')"
        />
        <h2 class="text-sm font-semibold">Quick click</h2>
      </div>
    </template>

    <div class="flex flex-col gap-3">
      <UFormField label="Mouse button" size="sm">
        <URadioGroup
          v-model="button"
          :items="buttonItems"
          indicator="hidden"
          variant="table"
          orientation="horizontal"
          size="sm"
        />
      </UFormField>

      <UFormField label="Interval" help="Milliseconds between clicks" size="sm">
        <UInputNumber v-model="intervalMs" size="sm" class="w-full" />
      </UFormField>

      <UFormField label="Toggle hotkey" size="sm">
        <HotkeyInput v-model="hotkey" />
      </UFormField>
    </div>
  </UCard>
</template>
