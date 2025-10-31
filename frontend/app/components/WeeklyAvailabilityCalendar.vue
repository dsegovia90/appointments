<script setup lang="ts">
import { weekdays, hours } from "~/utils/constants";

const grid = useTemplateRef("grid");
provide("grid", grid);
</script>

<template>
  <div
    class="grid grid-cols-7 ml-18 mt-10 sticky top-(--ui-header-height) bg-default z-20"
  >
    <div v-for="day in weekdays" :key="day.key">
      <div class="p-2 text-center">{{ day.label }}</div>
    </div>
  </div>
  <div
    ref="grid"
    class="grid grid-cols-7 pl-18 relative divide-x divide-neutral-600"
  >
    <div
      v-for="(hour, idx) in hours"
      :key="hour"
      class="flex gap-4 absolute left-0 w-full top-[480px] h-0 items-center"
      :style="{
        top: `${idx * 60}px`,
      }"
    >
      <span class="text-neutral-400 w-18">
        {{ hour }}
      </span>
      <USeparator />
    </div>

    <CalendarWeekday v-for="day in weekdays" :key="day.key" :weekday="day" />
  </div>
</template>
