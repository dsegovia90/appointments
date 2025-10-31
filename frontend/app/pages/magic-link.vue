<script setup lang="ts">
import { useUserStore } from "~/stores/user";

definePageMeta({
  layout: "client-facing",
});

const userStore = useUserStore();
const { token } = useRoute().query;

onMounted(() => {
  userStore.verifyMagicLink(token as string);
});
</script>

<template>
  <UContainer class="min-h-dvh flex items-center max-w-2xl">
    <div class="w-full text-center">
      <UProgress />
      <p class="mt-8">Verifying your token...</p>
    </div>
  </UContainer>
</template>
