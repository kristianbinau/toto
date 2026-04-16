<script setup lang="ts">
import { ref, watch } from "vue";
import type { ProfileEntry, Action, Repeat } from "../../lib/types";
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

function setId(v: string) {
  draft.value.script.id = v.trim() || "profile";
}

function setActions(v: Action[]) {
  draft.value.script.actions = v;
}

function setRepeat(v: Repeat) {
  draft.value.script.repeat = v;
}

function setHotkey(v: string | undefined) {
  draft.value.hotkey = v;
}

function save() {
  emit("save", JSON.parse(JSON.stringify(draft.value)));
}
</script>

<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <h2 class="text-sm font-semibold">Edit profile</h2>
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-x"
          @click="emit('cancel')"
        />
      </div>
    </template>

    <div class="flex flex-col gap-3">
      <div>
        <label class="text-xs text-neutral-500 block mb-1">Name / ID</label>
        <UInput
          :model-value="draft.script.id"
          @update:model-value="setId($event as string)"
          size="sm"
        />
      </div>

      <div>
        <label class="text-xs text-neutral-500 block mb-1">Repeat</label>
        <RepeatEditor :model-value="draft.script.repeat" @update:model-value="setRepeat" />
      </div>

      <div>
        <label class="text-xs text-neutral-500 block mb-1">Hotkey</label>
        <HotkeyInput :model-value="draft.hotkey" @update:model-value="setHotkey" />
      </div>

      <div>
        <label class="text-xs text-neutral-500 block mb-1">Actions</label>
        <ActionList :model-value="draft.script.actions" @update:model-value="setActions" />
      </div>
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
