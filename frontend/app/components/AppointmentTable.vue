<script setup lang="ts">
import type { Appointment } from "~/bindings/Appointment";
import type { DropdownMenuItem, TableColumn } from "@nuxt/ui";
import type { Row } from "@tanstack/vue-table";
import { parseAbsoluteToLocal } from "@internationalized/date";
const UBadge = resolveComponent("UBadge");
const UDropdownMenu = resolveComponent("UDropdownMenu");
const UButton = resolveComponent("UButton");

interface Props {
  page: number;
  limit: number;
  loading: boolean;
}

const toast = useToast();

const appointments = defineModel<Appointment[]>("appointments");
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
  {
    id: "actions",
    header: "Actions",
    cell: ({ row }) => {
      return h(
        "div",
        { class: "text-right" },
        h(
          UDropdownMenu,
          {
            content: {
              align: "end",
            },
            items: getActionItems(row),
            "aria-label": "Actions dropdown",
          },
          () =>
            h(UButton, {
              icon: "i-lucide-ellipsis-vertical",
              color: "neutral",
              variant: "ghost",
              class: "ml-auto",
              "aria-label": "Actions dropdown",
            }),
        ),
      );
    },
  },
];

const getActionItems = (row: Row<Appointment>): DropdownMenuItem[] => {
  return [
    {
      label: "Cancel Appointment",
      onSelect: async () => {
        try {
          const updatedAppointment = await api<Appointment>(
            `/api/appointments/cancel/${row.original.id}`,
            { method: "PATCH" },
          );
          const index = appointments.value?.findIndex(
            (appointment) => appointment.id === updatedAppointment.id,
          );
          if (appointments.value && index !== undefined && index !== -1) {
            appointments.value[index] = updatedAppointment;
          }

          toast.add({
            title: "Successfully canceled appointment.",
            color: "success",
          });
        } catch (error: unknown) {
          if (error instanceof Error) {
            toast.add({
              title: "Failed to cancel appointment.",
              description: error.message,
              color: "error",
            });
          } else {
            toast.add({
              title: "Failed to cancel appointment.",
              description: "An unexpected error occurred.",
              color: "error",
            });
          }

          console.error(error);
        }
      },
    },
  ];
};
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
