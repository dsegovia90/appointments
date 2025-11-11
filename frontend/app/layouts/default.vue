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
      label: "Availability",
      icon: "lucide:calendar-clock",
      to: "/dashboard/availability",
    },
    {
      label: "Integrations",
      icon: "lucide:link",
      to: "/dashboard/integrations",
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
      <UNavigationMenu :items="items" />
      <template #body>
        <UNavigationMenu
          :items="items"
          orientation="vertical"
          class="-mx-2.5"
        />
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
