<script setup lang="ts">
import type { Appointment } from "~/bindings/Appointment";
import type { AppointmentsResponse } from "~/bindings/AppointmentsResponse";
import type { TableColumn } from "@nuxt/ui";
import { parseAbsoluteToLocal } from "@internationalized/date";
import { useRouteQuery } from "@vueuse/router";

const appointmentTypes = useAppointmentTypeStore();
const loading = ref(true);
const toast = useToast();
const table = useTemplateRef("table");

const columns: TableColumn<Appointment>[] = [
  {
    accessorKey: "created_at",
    header: "Created",
    cell: ({ row }) => formatDate(row.original.created_at),
  },
  { accessorKey: "booker_name", header: "Name" },
  { accessorKey: "booker_phone", header: "Phone" },
  { accessorKey: "booker_timezone", header: "Timezone" },
  { accessorKey: "booker_email", header: "Email" },
  {
    accessorKey: "start_time",
    header: "Start Time",
    cell: ({ row }) => formatDateTime(row.original.start_time),
  },
  {
    id: "appointment_type",
    header: "Type",
    cell: ({ row }) =>
      appointmentTypes.appointmentTypes.get(row.original.appointment_type_id)
        ?.display_name || "Unknown",
  },
  {
    accessorKey: "duration",
    header: "Duration",
    cell: ({ row }) =>
      formatDuration(
        parseAbsoluteToLocal(row.original.endtime).compare(
          parseAbsoluteToLocal(row.original.start_time),
        ) /
          60 /
          1000,
      ),
  },
  { accessorKey: "status", header: "Status" },
  { id: "actions", header: "Actions" },
];

const page = useRouteQuery("page", "0", { transform: Number });
const limit = useRouteQuery("limit", "10", { transform: Number });
const from_date = useRouteQuery("from_date", null);
const to_date = useRouteQuery("to_date", null);
const status = useRouteQuery("status", null);
const appointment_type = useRouteQuery("appointment_type", null);

const appointments = ref<Appointment[]>([]);
const count = ref(0n);

const fetchAppointments = async () => {
  loading.value = true;
  const urlSearchParams = new URLSearchParams({
    page: page.value.toString(),
    limit: limit.value.toString(),
  });
  if (from_date.value) urlSearchParams.set("from_date", from_date.value);
  if (to_date.value) urlSearchParams.set("to_date", to_date.value);
  if (status.value) urlSearchParams.set("status", status.value);
  if (appointment_type.value)
    urlSearchParams.set("appointment_type", appointment_type.value);
  const response = await api<AppointmentsResponse>("/api/appointments", {
    urlSearchParams,
  });
  appointments.value = response.appointments;
  count.value = response.count;
  loading.value = false;
};

const pagination = computed(() => ({
  pageIndex: page.value,
  pageSize: limit.value,
}));

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
  <UDashboardPanel id="home">
    <template #header>
      <UDashboardNavbar title="Appointments">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="ml-auto">
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
      </div>
      <UTable
        ref="table"
        :columns="columns"
        :data="appointments"
        :pagination="pagination"
      >
      </UTable>
      <div class="flex justify-center border-t border-default pt-4">
        <UPagination
          :default-page="1"
          :items-per-page="limit"
          :total="Number(count)"
          @update:page="
            (p) => {
              page = p - 1;
              fetchAppointments();
            }
          "
        />
      </div>
    </template>
  </UDashboardPanel>
</template>
