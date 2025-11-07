<script setup lang="ts">
definePageMeta({
  layout: "client-facing",
});

const appointmentStore = useAppointmentTypeStore();
</script>

<template>
  <div class="flex min-h-dvh items-center justify-center">
    <div class="grid gap-8 w-full">
      <UPageCard
        v-for="type in appointmentStore.appointmentTypes.values()"
        :key="type.id"
        :title="type.display_name"
        :description="formatDuration(type.duration_in_minutes, 'long')"
        :to="`/appointment-type/${type.id}`"
        :ui="{ root: 'block md:max-w-2xl mx-auto w-full' }"
        icon="lucide:calendar-days"
        variant="soft"
      />
    </div>
  </div>
</template>
