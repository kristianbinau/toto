<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { useToast } from "@nuxt/ui/composables/useToast";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useProfilesStore } from "./stores/profiles";
import { setToastSink } from "./lib/toast";
import SimpleMode from "./components/SimpleMode.vue";
import ProfileList from "./components/advanced/ProfileList.vue";
import AboutDialog from "./components/common/AboutDialog.vue";

const store = useProfilesStore();
const toast = useToast();

const isSimple = computed(() => store.activeMode === "simple");
const runningCount = computed(() => store.runningIds.size);
const aboutOpen = ref(false);

let unlistenAbout: UnlistenFn | null = null;

function setMode(mode: "simple" | "advanced") {
  store.setActiveMode(mode);
}

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
        <div
          class="inline-flex rounded-md overflow-hidden border border-neutral-300 dark:border-neutral-700"
        >
          <UButton
            size="xs"
            :color="isSimple ? 'primary' : 'neutral'"
            :variant="isSimple ? 'solid' : 'ghost'"
            class="rounded-none"
            @click="setMode('simple')"
          >
            Simple
          </UButton>
          <UButton
            size="xs"
            :color="!isSimple ? 'primary' : 'neutral'"
            :variant="!isSimple ? 'solid' : 'ghost'"
            class="rounded-none"
            @click="setMode('advanced')"
          >
            Advanced
          </UButton>
        </div>
      </header>

      <main class="flex-1 overflow-y-auto p-3">
        <SimpleMode v-if="isSimple" />
        <ProfileList v-else />
      </main>

      <AboutDialog v-model:open="aboutOpen" />
    </div>
  </UApp>
</template>
