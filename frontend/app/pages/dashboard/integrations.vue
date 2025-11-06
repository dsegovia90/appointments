<script setup lang="ts">
const handleGoogleOAuthUrlClick = async () => {
  const response = await api<string>("/api/google_calendar/oauth_url");

  window.location.href = response;
};
</script>

<template>
  <div class="space-y-6">
    <UPageHeader>
      <template #title>
        <h1 class="text-4xl font-bold">Integrations</h1>
      </template>
      <template #description>
        <p class="text-sm text-neutral-100 mt-1">
          Add integrations to your appointments platform.
        </p>
      </template>
    </UPageHeader>

    <UPageBody>
      <h2 class="text-2xl font-bold">Google Calendar</h2>
      <UButton
        size="xl"
        class="bg-white rounded-sm cursor-pointer hover:bg-gray-200"
        icon="logos:google-icon"
        @click="handleGoogleOAuthUrlClick"
      >
        Sign in with Google
      </UButton>
    </UPageBody>
  </div>
</template>
