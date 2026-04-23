<script setup lang="ts">
import { computed } from "vue";
import { parseAccelerator } from "../../lib/accelerator";

const props = withDefaults(defineProps<{ accel?: string; size?: "sm" | "md" | "lg" }>(), {
  size: "sm",
});

const tokens = computed(() => parseAccelerator(props.accel ?? ""));
</script>

<template>
  <span v-if="tokens.length" class="inline-flex items-center gap-1">
    <template v-for="(tok, i) in tokens" :key="i">
      <span v-if="i > 0" class="text-neutral-400 dark:text-neutral-500 text-xs">+</span>
      <UKbd v-if="'value' in tok" :value="tok.value" :size="props.size" />
      <UKbd v-else :size="props.size">{{ tok.literal }}</UKbd>
    </template>
  </span>
</template>
