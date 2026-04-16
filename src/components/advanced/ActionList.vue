<script setup lang="ts">
import type { Action, ActionKind } from "../../lib/types";
import { defaultActionFor } from "../../lib/types";
import ActionRow from "./ActionRow.vue";

const props = defineProps<{ modelValue: Action[] }>();
const emit = defineEmits<{ "update:modelValue": [value: Action[]] }>();

function update(index: number, value: Action) {
  const next = props.modelValue.slice();
  next[index] = value;
  emit("update:modelValue", next);
}

function remove(index: number) {
  const next = props.modelValue.slice();
  next.splice(index, 1);
  emit("update:modelValue", next);
}

function move(index: number, delta: number) {
  const target = index + delta;
  if (target < 0 || target >= props.modelValue.length) return;
  const next = props.modelValue.slice();
  const [item] = next.splice(index, 1);
  next.splice(target, 0, item);
  emit("update:modelValue", next);
}

function add(kind: ActionKind) {
  emit("update:modelValue", [...props.modelValue, defaultActionFor(kind)]);
}

const addItems = [
  [
    { label: "Click", icon: "i-lucide-mouse-pointer-click", onSelect: () => add("Click") },
    { label: "Key", icon: "i-lucide-keyboard", onSelect: () => add("Key") },
    { label: "Move", icon: "i-lucide-move", onSelect: () => add("Move") },
    { label: "Delay", icon: "i-lucide-timer", onSelect: () => add("Delay") },
  ],
];
</script>

<template>
  <div class="flex flex-col gap-2">
    <ActionRow
      v-for="(action, i) in props.modelValue"
      :key="i"
      :model-value="action"
      :index="i"
      :total="props.modelValue.length"
      @update:model-value="update(i, $event)"
      @delete="remove(i)"
      @move="move(i, $event)"
    />
    <p
      v-if="props.modelValue.length === 0"
      class="text-xs text-neutral-500 italic text-center py-2"
    >
      No actions yet.
    </p>
    <UDropdownMenu :items="addItems">
      <UButton block size="xs" color="neutral" variant="outline" icon="i-lucide-plus">
        Add action
      </UButton>
    </UDropdownMenu>
  </div>
</template>
