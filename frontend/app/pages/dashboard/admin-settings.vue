<script setup lang="ts">
import type { AdminSettings } from "~/bindings/AdminSettings";
import GoogleCalendarAdminSettingsForm from "~/components/GoogleCalendarAdminSettingsForm.vue";
import { useAdminSettingsAPI } from "~/composables/useAdminSettingsAPI";

const adminSettings = ref<AdminSettings>();
const { fetch, update } = useAdminSettingsAPI();

onMounted(async () => {
  adminSettings.value = await fetch();
});

const handleUpdate = async (data: Partial<AdminSettings>) => {
  adminSettings.value = await update(data);
};
</script>

<template>
  <div class="flex-1">
    <UDashboardPanel>
      <template #header>
        <UDashboardNavbar title="Admin Settings">
          <template #leading> <UDashboardSidebarCollapse /></template>
        </UDashboardNavbar>
      </template>

      <template #body>
        <p>Update your admin settings and preferences</p>
        <LoadingLinear v-if="!adminSettings"> Loading... </LoadingLinear>
        <template v-else>
          <h2 class="text-2xl font-bold">General Settings</h2>
          <UFormField
            label="Allow new users to register?"
            description="They will register without admin privileges"
            size="xl"
          >
            <USwitch
              v-model="adminSettings.allow_new_registrations"
              unchecked-icon="i-lucide-x"
              checked-icon="i-lucide-check"
              default-value
              :label="
                adminSettings.allow_new_registrations!!
                  ? 'Allow new registrations'
                  : 'Disallow new registrations'
              "
              @update:model-value="
                (value) => handleUpdate({ allow_new_registrations: value })
              "
            />
          </UFormField>

          <USeparator />

          <h2 class="text-2xl font-bold">Notification Settings</h2>

          <GoogleCalendarAdminSettingsForm
            :initial-values="
              adminSettings.google_calendar_settings || undefined
            "
            @submit="
              (value) => handleUpdate({ google_calendar_settings: value })
            "
          />
        </template>
      </template>
    </UDashboardPanel>
  </div>
</template>
