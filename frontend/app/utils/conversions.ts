import type { DaysHoursMinutes } from "~/bindings/DaysHoursMinutes";

export const daysHoursMinutesToMinutes = (data: DaysHoursMinutes): number => {
  return data.days * 24 * 60 + data.hours * 60 + data.minutes;
};

export const minutesToDaysHoursMinutes = (
  minutes: number,
): DaysHoursMinutes => {
  const days = Math.floor(minutes / (24 * 60));
  const hours = Math.floor((minutes % (24 * 60)) / 60);
  const minutesLeft = minutes % 60;
  return { days, hours, minutes: minutesLeft };
};
