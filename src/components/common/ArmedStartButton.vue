<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { Script } from "../../lib/types";
import { useProfilesStore } from "../../stores/profiles";

const props = withDefaults(
  defineProps<{
    script: Script;
    size?: "xs" | "sm" | "md";
    block?: boolean;
  }>(),
  { size: "sm", block: false },
);

const store = useProfilesStore();
const GRACE_MS = 1000;

const running = computed(() => store.runningIds.has(props.script.id));
const arming = computed(() => {
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  store.armTick;
  return store.isArming(props.script.id);
});
const remainingSec = computed(() => {
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  store.armTick;
  return Math.ceil(store.armRemainingMs(props.script.id) / 1000);
});

const inGrace = ref(false);
let graceTimer: number | null = null;

watch(running, (isNow, wasBefore) => {
  if (isNow && !wasBefore) {
    inGrace.value = true;
    if (graceTimer !== null) clearTimeout(graceTimer);
    graceTimer = window.setTimeout(() => {
      inGrace.value = false;
      graceTimer = null;
    }, GRACE_MS);
  }
  if (!isNow && graceTimer !== null) {
    clearTimeout(graceTimer);
    graceTimer = null;
    inGrace.value = false;
  }
});

function onClick() {
  store.armOrCancel(JSON.parse(JSON.stringify(props.script)), 2000);
}

const label = computed(() => {
  if (arming.value) return `Starting ${remainingSec.value}…`;
  if (running.value) return "Stop";
  return "Start";
});

const color = computed<"primary" | "warning" | "error">(() => {
  if (arming.value) return "warning";
  if (running.value) return "error";
  return "primary";
});

const icon = computed(() => {
  if (arming.value) return "i-lucide-timer";
  if (running.value) return "i-lucide-square";
  return "i-lucide-play";
});

const variant = computed<"solid" | "soft">(() => (arming.value ? "soft" : "solid"));
</script>

<template>
  <UButton
    :color="color"
    :variant="variant"
    :icon="icon"
    :size="props.size"
    :block="props.block"
    :class="inGrace ? 'pointer-events-none opacity-90' : ''"
    :aria-disabled="inGrace || undefined"
    @click="onClick"
  >
    {{ label }}
  </UButton>
</template>
