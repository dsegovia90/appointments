<script setup lang="ts"></script>

<template>
  <UDashboardPanel id="availability">
    <template #header>
      <UDashboardNavbar title="Availability">
        <template #leading> <UDashboardSidebarCollapse /> </template
      ></UDashboardNavbar>
    </template>
    <template #body>
      <p>Manage your availability and their durations</p>
      <UserSettingsForm />
      <WeeklyAvailabilityCalendar />
    </template>
  </UDashboardPanel>
</template>
