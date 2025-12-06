<script setup lang="ts">
import type { DateRange } from "reka-ui";
import {
  DateFormatter,
  getLocalTimeZone,
  today,
} from "@internationalized/date";
import { Status } from "~/bindings/Status";

const df = new DateFormatter("en-US", {
  dateStyle: "medium",
});
const appointmentTypeStore = useAppointmentTypesAdminStore();
const appointmentTypeItems = computed(() =>
  Array.from(appointmentTypeStore.appointmentTypesMap.values()).map((type) => ({
    label: type.display_name,
    value: type.id.toString(),
  })),
);

defineProps<{
  loading: boolean;
}>();
const dateRange = defineModel<DateRange>("dateRange", { required: true });
const status = defineModel<Status>("status");
const appointmentType = defineModel<number>("appointmentType");

defineEmits<{
  (e: "refresh"): void;
}>();
</script>

<template>
  <div>
    <h2 class="text-xl font-bold mb-4">Filters</h2>
    <div class="flex gap-4 flex-col lg:flex-row flex-wrap">
      <UFieldGroup size="xl">
        <UPopover>
          <UButton
            class="cursor-pointer"
            color="neutral"
            variant="subtle"
            icon="i-lucide-calendar"
          >
            <template v-if="!dateRange.start && !dateRange.end">
              All time
            </template>
            <template v-else>
              From
              <template v-if="dateRange.start">
                {{
                  dateRange.start.compare(today(getLocalTimeZone())) === 0
                    ? "today"
                    : df.format(dateRange.start.toDate(getLocalTimeZone()))
                }}
              </template>
              <template v-else>past</template>
              to
              <template v-if="dateRange.end">
                {{
                  dateRange.end.compare(today(getLocalTimeZone())) === 0
                    ? "today"
                    : df.format(dateRange.end.toDate(getLocalTimeZone()))
                }}
              </template>
              <template v-else>future</template>
            </template>
          </UButton>

          <template #content>
            <UCalendar
              v-model="dateRange"
              class="p-2"
              :number-of-months="2"
              range
            />
          </template>
        </UPopover>
        <UButton
          v-if="dateRange.start || dateRange.end"
          class="cursor-pointer"
          color="neutral"
          variant="subtle"
          icon="lucide:x"
          @click="dateRange = { start: undefined, end: undefined }"
        />
      </UFieldGroup>
      <UFieldGroup size="xl">
        <USelect
          v-model="status"
          :items="Object.keys(Status)"
          class="cursor-pointer"
          placeholder="Status"
        />
        <UButton
          v-if="status"
          class="cursor-pointer"
          color="neutral"
          variant="subtle"
          icon="lucide:x"
          @click="status = undefined"
        />
      </UFieldGroup>
      <UFieldGroup size="xl">
        <USelect
          class="cursor-pointer"
          placeholder="Appointment Type"
          :model-value="
            appointmentType ? appointmentType.toString() : undefined
          "
          :items="appointmentTypeItems"
          @update:model-value="(e) => (appointmentType = Number(e))"
        />
        <UButton
          v-if="appointmentType"
          class="cursor-pointer"
          color="neutral"
          variant="subtle"
          icon="lucide:x"
          @click="appointmentType = undefined"
        />
      </UFieldGroup>
      <UFieldGroup class="lg:ml-auto" size="xl">
        <UButton
          icon="i-lucide-refresh-cw"
          color="neutral"
          variant="outline"
          :loading="loading"
          :disabled="loading"
          @click="$emit('refresh')"
        >
          Refresh
        </UButton>
      </UFieldGroup>
    </div>
  </div>
</template>
