<script setup lang="ts">
import { computed } from "vue";
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

const count = computed({
  get: () => (props.modelValue.mode === "Times" ? props.modelValue.count : 1),
  set: (v: number) => {
    const n = Math.max(1, Math.floor(Number(v) || 1));
    emit("update:modelValue", { mode: "Times", count: n });
  },
});
</script>

<template>
  <div class="flex items-center gap-2">
    <USelect
      :items="modeItems"
      :model-value="props.modelValue.mode"
      @update:model-value="setMode($event as string)"
      size="sm"
      class="flex-1"
    />
    <UInputNumber
      v-if="props.modelValue.mode === 'Times'"
      v-model="count"
      :min="1"
      size="sm"
      class="w-24"
    />
  </div>
</template>
