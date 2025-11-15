<script setup lang="ts">
import type { Appointment } from "~/bindings/Appointment";
import type { AppointmentsResponse } from "~/bindings/AppointmentsResponse";
import { useRouteQuery } from "@vueuse/router";
import type { DateRange } from "reka-ui";
import {
  type CalendarDate,
  getLocalTimeZone,
  today,
  parseDate,
} from "@internationalized/date";
import type { Status } from "~/bindings/Status";

const loading = ref(true);
const toast = useToast();

const page = useRouteQuery("page", "0", { transform: Number });
const limit = useRouteQuery("limit", "10", { transform: Number });
const from_date = useRouteQuery<string | undefined, CalendarDate | undefined>(
  "from_date",
  undefined,
  {
    transform: {
      get: (val) => (val ? parseDate(val) : undefined),
      set: (val) => (val ? val.toString() : undefined),
    },
  },
);
const to_date = useRouteQuery<string | undefined, CalendarDate | undefined>(
  "to_date",
  undefined,
  {
    transform: {
      get: (val) => (val ? parseDate(val) : undefined),
      set: (val) => (val ? val.toString() : undefined),
    },
  },
);
const status = useRouteQuery<Status>("status");
const appointment_type = useRouteQuery("appointment_type", null, {
  transform: Number,
});

const appointments = ref<Appointment[]>([]);
const count = ref(0n);

const fetchAppointments = async () => {
  try {
    loading.value = true;
    const urlSearchParams = new URLSearchParams({
      page: page.value.toString(),
      limit: limit.value.toString(),
    });
    if (from_date.value)
      urlSearchParams.set("from_date", from_date.value.toString());
    if (to_date.value) urlSearchParams.set("to_date", to_date.value.toString());
    if (status.value) urlSearchParams.set("status", status.value);
    if (appointment_type.value)
      urlSearchParams.set(
        "appointment_type",
        appointment_type.value.toString(),
      );
    const response = await api<AppointmentsResponse>("/api/appointments", {
      urlSearchParams,
    });
    appointments.value = response.appointments;
    count.value = response.count;
    loading.value = false;
  } catch (error) {
    toast.add({
      title: "Error",
      icon: "i-lucide-alert-circle",
      description: "Failed to fetch appointments",
    });
    console.error(error);
  }
};

const dateRange = computed<DateRange>(() => ({
  start: from_date.value,
  end: to_date.value,
}));

onMounted(async () => {
  from_date.value = today(getLocalTimeZone());
  await fetchAppointments();
});

watch([from_date, to_date, limit, status, appointment_type], async () => {
  page.value = 0;
  await fetchAppointments();
});
watch(page, async () => {
  await fetchAppointments();
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
      <AppointmentFilters
        v-model:status="status"
        v-model:appointment-type="appointment_type"
        :loading="loading"
        :date-range="dateRange"
        @update:date-range="
          (e) => {
            from_date = e?.start ? parseDate(e.start.toString()) : undefined;
            to_date = e?.end ? parseDate(e.end.toString()) : undefined;
            fetchAppointments();
          }
        "
        @refresh="fetchAppointments"
      />

      <AppointmentTable
        :appointments="appointments"
        :page="page"
        :limit="limit"
        :loading="loading"
      />
      <div class="flex justify-center border-t border-default pt-4 -mt-6">
        <UPagination
          :default-page="page + 1"
          :items-per-page="limit"
          :total="Number(count)"
          @update:page="
            (p) => {
              page = p - 1;
            }
          "
        />
      </div>
    </template>
  </UDashboardPanel>
</template>
