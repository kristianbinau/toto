<script setup lang="ts">
import { ref } from "vue";
import type { ProfileEntry } from "../../lib/types";
import { useProfilesStore } from "../../stores/profiles";
import ProfileCard from "./ProfileCard.vue";
import ProfileEditor from "./ProfileEditor.vue";
import QuickClickCard from "../QuickClickCard.vue";
import QuickEditor from "../QuickEditor.vue";

type EditTarget = { kind: "quick" } | { kind: "profile"; index: number };

const store = useProfilesStore();
const editing = ref<EditTarget | null>(null);

function beginEditProfile(i: number) {
  editing.value = { kind: "profile", index: i };
}

function beginEditQuick() {
  editing.value = { kind: "quick" };
}

function save(entry: ProfileEntry) {
  if (editing.value?.kind !== "profile") return;
  store.updateProfile(editing.value.index, entry);
  editing.value = null;
}

function back() {
  editing.value = null;
}

function create() {
  store.addProfile();
  editing.value = { kind: "profile", index: store.profiles.length - 1 };
}

async function remove(i: number) {
  await store.deleteProfile(i);
}
</script>

<template>
  <div class="flex flex-col gap-2">
    <template v-if="editing?.kind === 'quick'">
      <QuickEditor @back="back" />
    </template>
    <template v-else-if="editing?.kind === 'profile' && store.profiles[editing.index]">
      <ProfileEditor :entry="store.profiles[editing.index]!" @save="save" @cancel="back" />
    </template>
    <template v-else>
      <QuickClickCard @edit="beginEditQuick" />
      <USeparator
        v-if="store.profiles.length > 0"
        :label="store.profiles.length === 1 ? '1 profile' : `${store.profiles.length} profiles`"
        :ui="{ label: 'text-[10px] text-neutral-500' }"
      />
      <ProfileCard
        v-for="(entry, i) in store.profiles"
        :key="`${entry.script.id}-${i}`"
        :entry="entry"
        :index="i"
        @edit="beginEditProfile(i)"
        @duplicate="store.duplicateProfile(i)"
        @delete="remove(i)"
      />
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
    </template>
  </div>
</template>
