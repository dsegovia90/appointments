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
  <div class="space-y-6">
    <UPageHeader>
      <template #title>
        <h1 class="text-4xl font-bold">Admin Settings</h1>
      </template>
      <template #description>
        <p class="text-sm text-neutral-100 mt-1">
          Update your admin settings and preferences
        </p>
      </template>
    </UPageHeader>

    <UPageBody>
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
          :initial-values="adminSettings.google_calendar_settings || undefined"
          @submit="(value) => handleUpdate({ google_calendar_settings: value })"
        />
      </template>
    </UPageBody>
  </div>
</template>
