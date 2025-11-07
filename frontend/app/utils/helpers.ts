export const getAvailabilityTime = (minutes: number) => {
  return `${Math.floor(minutes / 60)}:${`${Math.floor(minutes % 60)}`.padStart(2, "0")}`;
};

// Format duration for display
export const formatDuration = (
  minutes: number,
  format: "short" | "long" = "short",
) => {
  const hourString = format === "short" ? "h" : " hours";
  const minuteString = format === "short" ? "m" : " minutes";
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  if (hours > 0) {
    return `${hours}${hourString} ${mins}${minuteString}`;
  }
  return `${mins}${minuteString}`;
};

// Format date
export const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString([], {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
};

export const formatDateTime = (dateString: string) => {
  return new Date(dateString).toLocaleDateString([], {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
  });
};

interface WindowCalculatorProps {
  value: number;
  min: number;
  max: number;
  step?: number;
}

export const windowCalculator = ({
  value,
  min,
  max,
  step = 5,
}: WindowCalculatorProps) => {
  return (
    Math.ceil(Math.round(Math.min(max, Math.max(min, value))) / step) * step
  );
};
