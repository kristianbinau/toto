<script setup lang="ts">
import { onBeforeUnmount, ref, watch } from "vue";
import { formatKeyEvent } from "../../lib/hotkeys";
import HotkeyDisplay from "./HotkeyDisplay.vue";

const props = defineProps<{ modelValue?: string }>();
const emit = defineEmits<{
  "update:modelValue": [value: string | undefined];
}>();

const capturing = ref(false);

function onKeydown(e: KeyboardEvent) {
  if (!capturing.value) return;
  e.preventDefault();
  e.stopPropagation();
  if (e.key === "Escape") {
    capturing.value = false;
    return;
  }
  const accel = formatKeyEvent(e);
  if (accel) {
    emit("update:modelValue", accel);
    capturing.value = false;
  }
}

function startCapture() {
  capturing.value = true;
}

function clear() {
  emit("update:modelValue", undefined);
}

watch(capturing, (on) => {
  if (on) {
    window.addEventListener("keydown", onKeydown, true);
  } else {
    window.removeEventListener("keydown", onKeydown, true);
  }
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", onKeydown, true);
});
</script>

<template>
  <div class="flex items-center gap-2">
    <template v-if="capturing">
      <div
        class="flex-1 min-h-7 px-2 py-1 rounded-md border border-warning-500 dark:border-warning-400 bg-warning-50 dark:bg-warning-950/40 text-xs text-warning-700 dark:text-warning-300 flex items-center justify-between gap-2"
        role="status"
        aria-live="polite"
      >
        <span class="animate-pulse">Press any key…</span>
        <span class="text-[10px] opacity-70">Esc to cancel</span>
      </div>
    </template>
    <template v-else-if="props.modelValue">
      <div
        class="flex-1 min-h-7 px-2 py-1 rounded-md border border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 flex items-center"
      >
        <HotkeyDisplay :accel="props.modelValue" />
      </div>
      <UButton
        size="xs"
        color="neutral"
        variant="ghost"
        icon="i-lucide-pencil"
        aria-label="Change hotkey"
        @click="startCapture"
      />
      <UButton
        size="xs"
        color="neutral"
        variant="ghost"
        icon="i-lucide-x"
        aria-label="Clear hotkey"
        @click="clear"
      />
    </template>
    <template v-else>
      <UButton
        size="xs"
        color="neutral"
        variant="outline"
        icon="i-lucide-keyboard"
        block
        @click="startCapture"
      >
        Set hotkey
      </UButton>
    </template>
  </div>
</template>
