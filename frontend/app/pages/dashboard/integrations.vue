<script setup lang="ts"></script>

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
      <GoogleCalendarIntegration />
    </UPageBody>
  </div>
</template>
