<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { ProfileEntry, Action, Repeat } from "../../lib/types";
import { DEFAULT_ARM_DELAY_MS } from "../../lib/types";
import ActionList from "./ActionList.vue";
import RepeatEditor from "./RepeatEditor.vue";
import HotkeyInput from "../common/HotkeyInput.vue";

const props = defineProps<{ entry: ProfileEntry }>();
const emit = defineEmits<{
  save: [entry: ProfileEntry];
  cancel: [];
}>();

const draft = ref<ProfileEntry>(JSON.parse(JSON.stringify(props.entry)));

watch(
  () => props.entry,
  (next) => {
    draft.value = JSON.parse(JSON.stringify(next));
  },
);

const id = computed({
  get: () => draft.value.script.id,
  set: (v: string) => {
    draft.value.script.id = v.trim() || "profile";
  },
});

function setActions(v: Action[]) {
  draft.value.script.actions = v;
}

function setRepeat(v: Repeat) {
  draft.value.script.repeat = v;
}

const hotkey = computed({
  get: () => draft.value.hotkey,
  set: (v: string | undefined) => {
    draft.value.hotkey = v;
  },
});

const armDelayMs = computed({
  get: () => draft.value.armDelayMs ?? DEFAULT_ARM_DELAY_MS,
  set: (v: number) => {
    const n = Number(v);
    draft.value.armDelayMs = Math.max(0, Math.min(10_000, Number.isFinite(n) ? n : 0));
  },
});

function save() {
  emit("save", JSON.parse(JSON.stringify(draft.value)));
}
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
          @click="emit('cancel')"
        />
        <h2 class="text-sm font-semibold">Edit profile</h2>
      </div>
    </template>

    <div class="flex flex-col gap-3">
      <UFormField label="Name" size="sm">
        <UInput v-model="id" size="sm" class="w-full" />
      </UFormField>

      <UFormField label="Repeat" size="sm">
        <RepeatEditor :model-value="draft.script.repeat" @update:model-value="setRepeat" />
      </UFormField>

      <UFormField label="Hotkey" size="sm">
        <HotkeyInput v-model="hotkey" />
      </UFormField>

      <UFormField
        label="Activation delay"
        help="Countdown before the script actually starts."
        size="sm"
      >
        <UInputNumber
          v-model="armDelayMs"
          :min="0"
          :max="10000"
          :step="500"
          size="sm"
          class="w-full"
        />
      </UFormField>

      <UFormField label="Actions" size="sm">
        <ActionList :model-value="draft.script.actions" @update:model-value="setActions" />
      </UFormField>
    </div>

    <template #footer>
      <div class="flex gap-2 justify-end">
        <UButton size="sm" color="neutral" variant="ghost" @click="emit('cancel')">
          Cancel
        </UButton>
        <UButton size="sm" color="primary" @click="save">Save</UButton>
      </div>
    </template>
  </UCard>
</template>
