<script setup lang="ts"></script>

<template>
  <UDashboardPanel>
    <template #header>
      <UDashboardNavbar title="Integrations">
        <template #leading> <UDashboardSidebarCollapse /> </template
      ></UDashboardNavbar>
    </template>

    <template #body>
      <p>Add integrations to your appointments platform.</p>
      <GoogleCalendarIntegration />
    </template>
  </UDashboardPanel>
</template>
