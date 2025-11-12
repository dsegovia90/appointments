<script setup lang="ts"></script>

<template>
  <div class="flex-1">
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
  </div>
</template>
