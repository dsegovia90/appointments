<script setup lang="ts">
import { weekdays, hours } from "~/utils/constants";

const grid = useTemplateRef("grid");
provide("grid", grid);
</script>

<template>
  <div ref="grid" class="grid grid-cols-7 mt-10 pl-18 min-w-4xl relative">
    <div
      v-for="day in weekdays"
      :key="day.key"
      class="sticky top-(--ui-header-height) bg-default z-20"
    >
      <div class="p-2 text-center truncate">{{ day.label }}</div>
    </div>

    <div class="absolute top-0 mt-10 left-0 w-full h-full">
      <div v-for="hour in hours" :key="hour" class="h-[60px] relative">
        <div class="flex gap-4 w-full items-center h-0">
          <span class="text-neutral-400 w-18">
            {{ hour }}
          </span>
          <USeparator />
        </div>
      </div>
    </div>

    <CalendarWeekday v-for="day in weekdays" :key="day.key" :weekday="day" />
  </div>
</template>
