<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui/runtime/types/index.js";
import { useUserStore } from "@/stores/user";

const userStore = useUserStore();

const items = computed<NavigationMenuItem[]>(() => {
  const items = [
    {
      label: "Appointments",
      icon: "lucide:calendar-check-2",
      to: "/dashboard",
    },
    {
      label: "Appointment Types",
      icon: "lucide:app-window",
      to: "/dashboard/appointment-types",
    },
    {
      label: "Weekly Availability",
      icon: "lucide:calendar-clock",
      to: "/dashboard/weekly-availability",
    },
  ];

  if (userStore.user?.role === "Admin") {
    items.push({
      label: "Admin Settings",
      icon: "lucide:settings",
      to: "/dashboard/admin-settings",
    });
  }

  return items;
});
</script>

<template>
  <UMain>
    <UHeader>
      <template #title> Admin Dashboard </template>
      <template #left>
        <UNavigationMenu :items="items" />
      </template>
      <template #right>
        <UColorModeButton />

        <UButton
          color="neutral"
          variant="ghost"
          @click="useUserStore().handleLogout"
          >Logout</UButton
        >
      </template>
    </UHeader>
    <UContainer>
      <slot />
    </UContainer>
  </UMain>
</template>
