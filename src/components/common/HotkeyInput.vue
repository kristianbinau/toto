<script setup lang="ts">
import { ref } from "vue";
import { formatKeyEvent } from "../../lib/hotkeys";

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

function clear() {
  emit("update:modelValue", undefined);
}

function startCapture() {
  capturing.value = true;
}
</script>

<template>
  <div class="flex items-center gap-2" @keydown="onKeydown">
    <UButton
      size="xs"
      :color="capturing ? 'warning' : 'neutral'"
      :variant="capturing ? 'solid' : 'outline'"
      icon="i-lucide-keyboard"
      @click="startCapture"
      tabindex="0"
    >
      {{ capturing ? "Press keys…" : props.modelValue || "Set hotkey" }}
    </UButton>
    <UButton
      v-if="props.modelValue && !capturing"
      size="xs"
      color="neutral"
      variant="ghost"
      icon="i-lucide-x"
      @click="clear"
      aria-label="Clear hotkey"
    />
  </div>
</template>
