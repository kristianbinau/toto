<script setup lang="ts">
import { computed } from "vue";
import type { ProfileEntry } from "../../lib/types";
import { summarizeScript } from "../../lib/types";
import { useProfilesStore } from "../../stores/profiles";
import ArmedStartButton from "../common/ArmedStartButton.vue";
import HotkeyDisplay from "../common/HotkeyDisplay.vue";

const props = defineProps<{ entry: ProfileEntry; index: number }>();
defineEmits<{
  edit: [];
  duplicate: [];
  delete: [];
}>();

const store = useProfilesStore();
const summary = computed(() => summarizeScript(props.entry.script));
</script>

<template>
  <UCard :ui="{ body: 'p-3 sm:p-3' }">
    <div class="flex items-center gap-2">
      <div class="flex-1 min-w-0">
        <h3 class="text-sm font-semibold truncate">{{ props.entry.script.id }}</h3>
        <p class="text-xs text-neutral-500 truncate">{{ summary }}</p>
        <div v-if="props.entry.hotkey" class="mt-1">
          <HotkeyDisplay :accel="props.entry.hotkey" />
        </div>
      </div>
      <div class="flex items-center gap-0.5 shrink-0">
        <ArmedStartButton :script="props.entry.script" size="xs" />
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-pencil"
          @click="$emit('edit')"
          aria-label="Edit"
        />
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-copy"
          @click="$emit('duplicate')"
          aria-label="Duplicate"
        />
        <UButton
          size="xs"
          color="error"
          variant="ghost"
          icon="i-lucide-trash-2"
          @click="$emit('delete')"
          aria-label="Delete"
        />
      </div>
    </div>
  </UCard>
</template>
