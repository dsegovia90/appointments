<script setup lang="ts">
import { weekdays, hours } from "~/utils/constants";
</script>

<template>
  <h2 class="text-2xl mx-auto">Weekly Availability Calendar</h2>
  <div class="max-w-full">
    <div ref="grid" class="pl-18 min-w-4xl relative">
      <div class="absolute top-10 left-0 w-full">
        <div v-for="hour in hours" :key="hour" class="h-[60px] relative">
          <div class="flex gap-4 w-full items-center h-0">
            <span class="text-neutral-400 w-18">
              {{ hour }}
            </span>
            <USeparator />
          </div>
        </div>
      </div>

      <div class="grid grid-cols-7">
        <CalendarWeekday
          v-for="day in weekdays"
          :key="day.key"
          :weekday="day"
        />
      </div>
    </div>
  </div>
</template>
