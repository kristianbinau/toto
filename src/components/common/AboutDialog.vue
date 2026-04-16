<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getVersion, getName } from "@tauri-apps/api/app";

defineProps<{ open: boolean }>();
defineEmits<{ "update:open": [value: boolean] }>();

const appName = ref("toto");
const appVersion = ref("");

onMounted(async () => {
  try {
    appName.value = await getName();
    appVersion.value = await getVersion();
  } catch {
    // Running outside Tauri (e.g. `npm run dev`) — keep defaults.
  }
});
</script>

<template>
  <UModal
    :open="open"
    :title="appName"
    :description="appVersion ? `Version ${appVersion}` : undefined"
    @update:open="$emit('update:open', $event)"
  >
    <template #body>
      <div class="flex flex-col gap-3 text-sm text-neutral-600 dark:text-neutral-300">
        <p>
          A small autoclicker built with Tauri, Vue, and a custom Rust engine.
        </p>
        <p class="text-xs text-neutral-500">
          Simple mode for one-button clicking. Advanced mode for multi-action
          scripts with per-profile hotkeys.
        </p>
      </div>
    </template>
  </UModal>
</template>
