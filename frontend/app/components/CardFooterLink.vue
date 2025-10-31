<script setup lang="ts">
interface Props {
  to: string;
  text: string;
  linkText: string;
}

defineProps<Props>();
</script>

<template>
  <div>
    {{ text }}
    <ULink :to="to" class="text-primary font-medium">{{ linkText }}</ULink
    >.
  </div>
</template>
