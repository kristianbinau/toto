<script setup lang="ts">
import type { Repeat } from "../../lib/types";

const props = defineProps<{ modelValue: Repeat }>();
const emit = defineEmits<{ "update:modelValue": [value: Repeat] }>();

const modeItems = [
  { label: "Once", value: "Once" },
  { label: "Times", value: "Times" },
  { label: "Infinite", value: "Infinite" },
];

function setMode(mode: string) {
  if (mode === "Times") {
    emit("update:modelValue", { mode: "Times", count: 1 });
  } else if (mode === "Once") {
    emit("update:modelValue", { mode: "Once" });
  } else {
    emit("update:modelValue", { mode: "Infinite" });
  }
}

function setCount(v: number) {
  const n = Math.max(1, Math.floor(Number(v) || 1));
  emit("update:modelValue", { mode: "Times", count: n });
}
</script>

<template>
  <div class="flex items-center gap-2">
    <USelect
      :items="modeItems"
      :model-value="props.modelValue.mode"
      @update:model-value="setMode($event as string)"
      size="xs"
      class="flex-1"
    />
    <UInput
      v-if="props.modelValue.mode === 'Times'"
      type="number"
      :min="1"
      :model-value="props.modelValue.count"
      @update:model-value="setCount($event as number)"
      size="xs"
      class="w-20"
    />
  </div>
</template>
