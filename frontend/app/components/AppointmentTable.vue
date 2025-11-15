<script setup lang="ts">
import type { Appointment } from "~/bindings/Appointment";
import type { TableColumn } from "@nuxt/ui";
import { parseAbsoluteToLocal } from "@internationalized/date";
const UBadge = resolveComponent("UBadge");

interface Props {
  appointments: Appointment[];
  page: number;
  limit: number;
  loading: boolean;
}

const props = defineProps<Props>();
const appointmentTypes = useAppointmentTypesAdminStore();

const pagination = computed(() => ({
  pageIndex: props.page,
  pageSize: props.limit,
}));

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
      appointmentTypes.appointmentTypesMap.get(row.original.appointment_type_id)
        ?.display_name || "Unknown",
  },
  {
    accessorKey: "duration",
    header: "Duration",
    cell: ({ row }) =>
      h(UBadge, {
        icon: "",
        color: "neutral",
        variant: "subtle",
        label: formatDuration(
          parseAbsoluteToLocal(row.original.endtime).compare(
            parseAbsoluteToLocal(row.original.start_time),
          ) /
            60 /
            1000,
        ),
      }),
  },
  { accessorKey: "status", header: "Status" },
  { id: "actions", header: "Actions" },
];
</script>
>
<template>
  <UTable
    ref="table"
    :columns="columns"
    :data="appointments"
    :pagination="pagination"
    :loading="loading"
  />
</template>
