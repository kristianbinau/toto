<script setup lang="ts">
import { ref } from "vue";
import type { ProfileEntry } from "../../lib/types";
import { useProfilesStore } from "../../stores/profiles";
import ProfileCard from "./ProfileCard.vue";
import ProfileEditor from "./ProfileEditor.vue";

const store = useProfilesStore();
const editingIndex = ref<number | null>(null);

function beginEdit(i: number) {
  editingIndex.value = i;
}

function save(entry: ProfileEntry) {
  if (editingIndex.value === null) return;
  store.updateProfile(editingIndex.value, entry);
  editingIndex.value = null;
}

function cancel() {
  editingIndex.value = null;
}

function create() {
  store.addProfile();
  editingIndex.value = store.profiles.length - 1;
}

async function remove(i: number) {
  await store.deleteProfile(i);
}
</script>

<template>
  <div class="flex flex-col gap-2">
    <template v-if="editingIndex !== null && store.profiles[editingIndex]">
      <ProfileEditor
        :entry="store.profiles[editingIndex]!"
        @save="save"
        @cancel="cancel"
      />
    </template>
    <template v-else>
      <ProfileCard
        v-for="(entry, i) in store.profiles"
        :key="`${entry.script.id}-${i}`"
        :entry="entry"
        :index="i"
        @edit="beginEdit(i)"
        @duplicate="store.duplicateProfile(i)"
        @delete="remove(i)"
      />
      <p
        v-if="store.profiles.length === 0"
        class="text-xs text-neutral-500 italic text-center py-2"
      >
        No profiles yet. Create one below.
      </p>
      <UButton
        block
        size="sm"
        color="primary"
        variant="outline"
        icon="i-lucide-plus"
        @click="create"
      >
        New profile
      </UButton>
      <UButton
        v-if="store.runningIds.size > 0"
        block
        size="sm"
        color="error"
        variant="soft"
        icon="i-lucide-square"
        @click="store.stopAll()"
      >
        Stop all ({{ store.runningIds.size }})
      </UButton>
    </template>
  </div>
</template>
