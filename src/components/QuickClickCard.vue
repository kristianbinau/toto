<script setup lang="ts">
import { computed } from "vue";
import { useProfilesStore } from "../stores/profiles";
import ArmedStartButton from "./common/ArmedStartButton.vue";
import HotkeyDisplay from "./common/HotkeyDisplay.vue";

defineEmits<{ edit: [] }>();

const store = useProfilesStore();

const script = computed(() => store.simpleScript());
const summary = computed(() => {
  const { button, intervalMs } = store.simpleConfig;
  return `${button} · every ${intervalMs} ms`;
});
</script>

<template>
  <UCard :ui="{ body: 'p-3 sm:p-3' }">
    <div class="flex items-center gap-2">
      <UIcon name="i-lucide-pin" class="text-primary-500 shrink-0" />
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2">
          <h3 class="text-sm font-semibold truncate">Quick click</h3>
        </div>
        <p class="text-xs text-neutral-500 truncate">{{ summary }}</p>
        <div v-if="store.simpleConfig.hotkey" class="mt-1">
          <HotkeyDisplay :accel="store.simpleConfig.hotkey" />
        </div>
      </div>
      <div class="flex items-center gap-1 shrink-0">
        <ArmedStartButton :script="script" size="xs" />
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-pencil"
          aria-label="Edit quick click"
          @click="$emit('edit')"
        />
      </div>
    </div>
  </UCard>
</template>
