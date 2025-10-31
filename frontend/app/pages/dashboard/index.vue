<script setup lang="ts">
import type { Appointment } from "~/bindings/Appointment";
import { useUserStore } from "~/stores/user";
import type { TableColumn } from "@nuxt/ui";
import { parseAbsoluteToLocal } from "@internationalized/date";

const userStore = useUserStore();
const appointmentTypes = useAppointmentTypeStore();
const loading = ref(true);
const toast = useToast();

const columns: TableColumn<Appointment>[] = [
  { accessorKey: "created_at", header: "Created" },
  // { accessorKey: "updated_at", header: "Updated At" },
  { accessorKey: "booker_name", header: "Name" },
  { accessorKey: "booker_phone", header: "Phone" },
  { accessorKey: "booker_timezone", header: "Timezone" },
  { accessorKey: "booker_email", header: "Email" },
  { accessorKey: "start_time", header: "Start Time" },
  { id: "appointment_type", header: "Type" },
  { accessorKey: "duration", header: "Duration" },
  { accessorKey: "status", header: "Status" },
  { id: "actions", header: "Actions" },
];

const appointments = ref<Appointment[]>([]);

const fetchAppointments = async () => {
  loading.value = true;
  appointments.value = await api<Appointment[]>("/api/appointments");
  loading.value = false;
};

onMounted(async () => {
  try {
    await fetchAppointments();
  } catch (error) {
    toast.add({
      title: "Error",
      icon: "i-lucide-alert-circle",
      description: "Failed to fetch appointments",
    });
    console.error(error);
  }
});
</script>

<template>
  <div>
    <UPageHeader :title="`Welcome, ${userStore.user?.name}!`">
      <template #links>
        <UButton
          icon="i-lucide-refresh-cw"
          color="neutral"
          variant="outline"
          :loading="loading"
          :disabled="loading"
          @click="fetchAppointments"
        >
          Refresh
        </UButton>
      </template>
    </UPageHeader>

    <UTable :columns="columns" :data="appointments">
      <template #created_at-cell="{ row }">
        <span class="text-sm">
          {{ formatDate(row.original.created_at) }}
        </span>
      </template>
      <template #start_time-cell="{ row }">
        <span class="text-sm">
          {{ formatDateTime(row.original.start_time) }}
        </span>
      </template>
      <template #appointment_type-cell="{ row }">
        <span class="text-sm">
          {{
            appointmentTypes.appointmentTypes.get(
              row.original.appointment_type_id,
            )?.display_name || "Unknown"
          }}
        </span>
      </template>
      <template #duration-cell="{ row }">
        <span class="text-sm">
          {{
            parseAbsoluteToLocal(row.original.endtime).compare(
              parseAbsoluteToLocal(row.original.start_time),
            ) /
            60 /
            1000
          }}
          mins
        </span>
      </template>
    </UTable>
  </div>
</template>
