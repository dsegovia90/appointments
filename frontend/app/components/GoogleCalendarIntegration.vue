<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";
import type { CalendarEntry } from "~/bindings/CalendarEntry";
import type { CalendarSettingParams } from "~/bindings/CalendarSettingParams";
import type { CalendarSettingType } from "~/bindings/CalendarSettingType";
import type { CalendarSettingsResponse } from "~/bindings/CalendarSettingsResponse";
import { useAPI } from "~/composables/useAPI";

const UIcon = resolveComponent("UIcon");
const { data: calendars, isFetching: isFetchingCalendars } = useAPI<
  CalendarEntry[]
>("/api/google_calendar/get_calendars").json();
const checkForCollision = ref<Set<string>>(new Set());
const createEventOnAppointment = ref<Set<string>>(new Set());
const toast = useToast();

const handleGoogleOAuthUrlClick = async () => {
  const response = await api<string>("/api/google_calendar/oauth_url");

  window.location.href = response;
};

const setCheckboxes = (
  response: CalendarSettingsResponse,
  withToast = false,
) => {
  checkForCollision.value = new Set(
    response.calendars_for_collision_check || [],
  );
  createEventOnAppointment.value = new Set(
    response.calendars_for_event_handling || [],
  );

  if (withToast) {
    toast.add({
      title: "Success",
      description: "Calendars updated successfully",
    });
  }
};

onMounted(async () => {
  const response = await api<CalendarSettingsResponse>(
    "/api/google_calendar/get_settings",
  );

  setCheckboxes(response);
});

// Table columns configuration
const columns: TableColumn<CalendarEntry>[] = [
  {
    accessorKey: "summary",
    header: "Calendar Name",
  },
  {
    accessorKey: "access_role",
    header: "Access Role",
  },
  {
    accessorKey: "primary",
    header: "Primary",
    cell: ({ row }) => {
      return h(UIcon, {
        class: "",
        name: row.original.primary ? "lucide:circle-check" : "lucide:circle",
        size: "16",
      });
    },
  },
  {
    id: "collision",
    header: "Check for Collision?",
  },
  {
    id: "create_event",
    header: "Create Event on Appointment?",
  },
];

const handleCalendarSettingUpdate = async (
  val: boolean | "indeterminate",
  calendar_id: string,
  setting_type: CalendarSettingType,
) => {
  const url = val
    ? "/api/google_calendar/add_calendar"
    : "/api/google_calendar/remove_calendar";
  const response = await api<CalendarSettingsResponse, CalendarSettingParams>(
    url,
    {
      method: "POST",
      body: { calendar_id, setting_type },
    },
  );
  setCheckboxes(response, true);
};
</script>

<template>
  <div>
    <h2>Google Calendar Integration</h2>
    <UButton
      size="xl"
      class="bg-white rounded-sm cursor-pointer hover:bg-gray-200"
      icon="logos:google-icon"
      @click="handleGoogleOAuthUrlClick"
    >
      Sign in with Google
    </UButton>
    <UTable
      :ui="{ tr: 'transition-opacity hover:opacity-80' }"
      :columns="columns"
      :data="calendars"
      :loading="isFetchingCalendars"
    >
      <template #collision-cell="{ row }">
        <UCheckbox
          :model-value="checkForCollision.has(row.original.id)"
          @update:model-value="
            (val) =>
              handleCalendarSettingUpdate(
                val,
                row.original.id,
                'CollisionCheck',
              )
          "
        />
      </template>
      <template #create_event-cell="{ row }">
        <UCheckbox
          :model-value="createEventOnAppointment.has(row.original.id)"
          @update:model-value="
            (val) =>
              handleCalendarSettingUpdate(val, row.original.id, 'EventHandling')
          "
        />
      </template>
    </UTable>
  </div>
</template>
