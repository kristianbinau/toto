<script setup lang="ts">
import type { Action, Key } from "../../lib/types";

const props = defineProps<{
  modelValue: Action;
  index: number;
  total: number;
}>();
const emit = defineEmits<{
  "update:modelValue": [value: Action];
  delete: [];
  move: [delta: number];
}>();

const buttonItems = [
  { label: "Left", value: "Left" },
  { label: "Right", value: "Right" },
  { label: "Middle", value: "Middle" },
];
const directionItems = [
  { label: "Click", value: "Click" },
  { label: "Press", value: "Press" },
  { label: "Release", value: "Release" },
];
const coordItems = [
  { label: "Relative", value: "Relative" },
  { label: "Absolute", value: "Absolute" },
];
const keyNameItems = [
  { label: "Unicode char", value: "Unicode" },
  { label: "Return", value: "Return" },
  { label: "Tab", value: "Tab" },
  { label: "Space", value: "Space" },
  { label: "Escape", value: "Escape" },
  { label: "Backspace", value: "Backspace" },
];

function update(patch: Partial<Action>) {
  emit("update:modelValue", { ...props.modelValue, ...patch } as Action);
}

function setKey(name: string) {
  const key: Key =
    name === "Unicode"
      ? { name: "Unicode", value: "a" }
      : ({ name } as Key);
  if (props.modelValue.type !== "Key") return;
  emit("update:modelValue", { ...props.modelValue, key });
}

function setUnicode(v: string) {
  if (props.modelValue.type !== "Key") return;
  const ch = (v ?? "").slice(0, 1) || "a";
  emit("update:modelValue", {
    ...props.modelValue,
    key: { name: "Unicode", value: ch },
  });
}

function currentKeyName(): string {
  if (props.modelValue.type !== "Key") return "Return";
  return props.modelValue.key.name;
}
</script>

<template>
  <div class="border border-neutral-200 dark:border-neutral-800 rounded-md p-2 flex flex-col gap-2">
    <div class="flex items-center gap-1">
      <UBadge color="neutral" variant="subtle" size="sm" class="font-mono">
        {{ props.modelValue.type }}
      </UBadge>
      <div class="flex-1" />
      <UButton
        size="xs"
        color="neutral"
        variant="ghost"
        icon="i-lucide-arrow-up"
        :disabled="props.index === 0"
        @click="emit('move', -1)"
        aria-label="Move up"
      />
      <UButton
        size="xs"
        color="neutral"
        variant="ghost"
        icon="i-lucide-arrow-down"
        :disabled="props.index === props.total - 1"
        @click="emit('move', 1)"
        aria-label="Move down"
      />
      <UButton
        size="xs"
        color="error"
        variant="ghost"
        icon="i-lucide-trash-2"
        @click="emit('delete')"
        aria-label="Delete action"
      />
    </div>

    <template v-if="props.modelValue.type === 'Click'">
      <div class="flex gap-2">
        <USelect
          :items="buttonItems"
          :model-value="props.modelValue.button"
          @update:model-value="update({ button: $event as any })"
          size="xs"
          class="flex-1"
        />
        <USelect
          :items="directionItems"
          :model-value="props.modelValue.direction"
          @update:model-value="update({ direction: $event as any })"
          size="xs"
          class="flex-1"
        />
      </div>
    </template>

    <template v-else-if="props.modelValue.type === 'Key'">
      <div class="flex gap-2">
        <USelect
          :items="keyNameItems"
          :model-value="currentKeyName()"
          @update:model-value="setKey($event as string)"
          size="xs"
          class="flex-1"
        />
        <USelect
          :items="directionItems"
          :model-value="props.modelValue.direction"
          @update:model-value="update({ direction: $event as any })"
          size="xs"
          class="flex-1"
        />
      </div>
      <UInput
        v-if="props.modelValue.key.name === 'Unicode'"
        :model-value="props.modelValue.key.value"
        @update:model-value="setUnicode($event as string)"
        placeholder="char"
        size="xs"
        :maxlength="1"
      />
    </template>

    <template v-else-if="props.modelValue.type === 'Move'">
      <div class="flex gap-2">
        <UInput
          type="number"
          :model-value="props.modelValue.x"
          @update:model-value="update({ x: Number($event) || 0 })"
          size="xs"
          class="flex-1"
          placeholder="x"
        />
        <UInput
          type="number"
          :model-value="props.modelValue.y"
          @update:model-value="update({ y: Number($event) || 0 })"
          size="xs"
          class="flex-1"
          placeholder="y"
        />
        <USelect
          :items="coordItems"
          :model-value="props.modelValue.coord"
          @update:model-value="update({ coord: $event as any })"
          size="xs"
          class="flex-1"
        />
      </div>
    </template>

    <template v-else-if="props.modelValue.type === 'Delay'">
      <UInput
        type="number"
        :min="0.1"
        :step="0.1"
        :model-value="props.modelValue.ms"
        @update:model-value="update({ ms: Math.max(0.1, Number($event) || 0.1) })"
        size="xs"
        placeholder="ms"
      />
    </template>
  </div>
</template>
