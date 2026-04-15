<script setup lang="ts">
import { computed } from "vue";
import type { ProfileEntry } from "../../lib/types";
import { summarizeScript } from "../../lib/types";
import { useProfilesStore } from "../../stores/profiles";

const props = defineProps<{ entry: ProfileEntry; index: number }>();
const emit = defineEmits<{
  edit: [];
  duplicate: [];
  delete: [];
}>();

const store = useProfilesStore();
const running = computed(() => store.runningIds.has(props.entry.script.id));
const summary = computed(() => summarizeScript(props.entry.script));

async function toggle() {
  await store.toggleScript(JSON.parse(JSON.stringify(props.entry.script)));
}
</script>

<template>
  <UCard>
    <div class="flex items-start gap-2">
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2">
          <h3 class="text-sm font-semibold truncate">{{ props.entry.script.id }}</h3>
          <UBadge
            v-if="running"
            color="success"
            variant="solid"
            size="sm"
          >
            Running
          </UBadge>
        </div>
        <p class="text-xs text-neutral-500 truncate">{{ summary }}</p>
        <p v-if="props.entry.hotkey" class="text-xs text-neutral-500 mt-0.5">
          <UKbd>{{ props.entry.hotkey }}</UKbd>
        </p>
      </div>
    </div>
    <template #footer>
      <div class="flex gap-1 justify-end">
        <UButton
          size="xs"
          :color="running ? 'error' : 'primary'"
          :icon="running ? 'i-lucide-square' : 'i-lucide-play'"
          @click="toggle"
        >
          {{ running ? "Stop" : "Start" }}
        </UButton>
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-pencil"
          @click="emit('edit')"
          aria-label="Edit"
        />
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-copy"
          @click="emit('duplicate')"
          aria-label="Duplicate"
        />
        <UButton
          size="xs"
          color="error"
          variant="ghost"
          icon="i-lucide-trash-2"
          @click="emit('delete')"
          aria-label="Delete"
        />
      </div>
    </template>
  </UCard>
</template>
