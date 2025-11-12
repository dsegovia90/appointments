<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui/runtime/types/index.js";
import { useUserStore } from "@/stores/user";

const userStore = useUserStore();
const open = ref(false);

const topLinks = computed<NavigationMenuItem[]>(() => {
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

const bottomLinks = ref<NavigationMenuItem[]>([
  {
    label: "Github",
    icon: "i-lucide-github",
    to: "https://github.com/dsegovia90/appointments",
    target: "_blank",
  },
  {
    label: "Bug Report",
    icon: "i-lucide-bug",
    to: "https://github.com/dsegovia90/appointments/issues",
    target: "_blank",
  },
]);

const groups = computed(() => [
  {
    id: "links",
    label: "Go to",
    // @ts-expect-error concatenated links should not error
    items: [...topLinks.value, ...bottomLinks.value],
  },
]);
</script>

<template>
  <UDashboardGroup unit="rem">
    <UDashboardSidebar
      id="default"
      v-model:open="open"
      collapsible
      resizable
      class="bg-elevated/25"
      :ui="{ footer: 'lg:border-t lg:border-default' }"
    >
      <template #header="{ collapsed }">
        <div
          :class="collapsed ? 'px-1.5' : 'px-2.5'"
          class="flex items-center gap-2"
        >
          <UIcon class="size-5" name="lucide:layout-dashboard" />
          <h2 v-if="!collapsed">Admin Dashboard</h2>
        </div>
      </template>

      <template #default="{ collapsed }">
        <UDashboardSearchButton
          :collapsed="collapsed"
          class="bg-transparent ring-default"
        />

        <UNavigationMenu
          :collapsed="collapsed"
          :items="topLinks"
          orientation="vertical"
          tooltip
          popover
        />

        <UNavigationMenu
          :collapsed="collapsed"
          :items="bottomLinks"
          orientation="vertical"
          tooltip
          class="mt-auto"
        />
      </template>

      <!-- <template #footer="{ collapsed }">
        <UserMenu :collapsed="collapsed" />
      </template> -->
    </UDashboardSidebar>

    <UDashboardSearch :groups="groups" />

    <slot />
  </UDashboardGroup>
</template>
