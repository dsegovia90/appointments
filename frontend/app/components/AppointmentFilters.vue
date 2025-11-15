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

defineProps<{
  loading: boolean;
}>();
const dateRange = defineModel<DateRange>("dateRange", { required: true });
const status = defineModel<Status>("status");

defineEmits<{
  (e: "refresh"): void;
}>();
</script>

<template>
  <div>
    <h2 class="text-xl font-bold mb-4">Filters</h2>
    <div class="flex gap-4">
      <UFieldGroup>
        <UPopover>
          <UButton color="neutral" variant="subtle" icon="i-lucide-calendar">
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
          class="cursor-pointer"
          color="neutral"
          variant="subtle"
          icon="lucide:x"
          @click="dateRange = { start: undefined, end: undefined }"
        />
      </UFieldGroup>
      <UFieldGroup>
        <USelect
          v-model="status"
          :items="Object.keys(Status)"
          placeholder="Status"
        />
        <UButton
          class="cursor-pointer"
          color="neutral"
          variant="subtle"
          icon="lucide:x"
          :disabled="!status"
          @click="status = undefined"
        />
      </UFieldGroup>
      <UButton
        class="ml-auto"
        icon="i-lucide-refresh-cw"
        color="neutral"
        variant="outline"
        :loading="loading"
        :disabled="loading"
        @click="$emit('refresh')"
      >
        Refresh
      </UButton>
    </div>
  </div>
</template>
