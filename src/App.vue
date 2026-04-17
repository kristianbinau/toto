<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { useToast } from "@nuxt/ui/composables/useToast";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useProfilesStore } from "./stores/profiles";
import { setToastSink } from "./lib/toast";
import ProfileList from "./components/advanced/ProfileList.vue";
import AboutDialog from "./components/common/AboutDialog.vue";

const store = useProfilesStore();
const toast = useToast();

const runningCount = computed(() => store.runningIds.size);
const aboutOpen = ref(false);

let unlistenAbout: UnlistenFn | null = null;

onMounted(async () => {
  setToastSink((opts) => toast.add(opts));
  try {
    unlistenAbout = await listen("show-about", () => {
      aboutOpen.value = true;
    });
  } catch {
    // Running outside Tauri — ignore.
  }
});

onBeforeUnmount(() => {
  setToastSink(null);
  unlistenAbout?.();
  unlistenAbout = null;
});
</script>

<template>
  <UApp>
    <div class="flex flex-col h-screen">
      <header
        class="flex items-center gap-2 px-3 py-2 border-b border-neutral-200 dark:border-neutral-800 shrink-0"
      >
        <h1 class="text-sm font-semibold">toto</h1>
        <UButton
          size="xs"
          color="neutral"
          variant="ghost"
          icon="i-lucide-info"
          aria-label="About toto"
          @click="aboutOpen = true"
        />
        <div class="flex-1" />
        <UBadge v-if="runningCount > 0" color="success" variant="solid" size="sm">
          {{ runningCount }} running
        </UBadge>
        <UButton
          v-if="runningCount > 0"
          size="xs"
          color="error"
          variant="soft"
          icon="i-lucide-square"
          aria-label="Stop all"
          @click="store.stopAll()"
        >
          Stop all
        </UButton>
      </header>

      <main class="flex-1 overflow-y-auto p-3">
        <ProfileList />
      </main>

      <AboutDialog v-model:open="aboutOpen" />
    </div>
  </UApp>
</template>
