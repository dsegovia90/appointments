<script setup lang="ts"></script>

<template>
  <div>
    <UPageHeader>
      <template #title>
        <h1 class="text-4xl font-bold">Availability</h1>
      </template>
      <template #description>
        <p class="text-sm text-neutral-100 mt-1">
          Manage your availability and their durations
        </p>
      </template>
    </UPageHeader>
    <UserSettingsForm />

    <USeparator class="my-4" />
    <WeeklyAvailabilityCalendar />
  </div>
</template>
