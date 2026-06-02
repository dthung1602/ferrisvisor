import type { ProcessInfo } from "$lib/api/process";
import { localstorage } from "$lib/index";

export function formatDate(date: string) {
  if (!date) return "N/A";
  const timeZone = localstorage.get(localstorage.TIMEZONE, "UTC");
  const instant = window.Temporal.Instant.from(date);
  const dt = instant.toZonedDateTimeISO(timeZone);
  const year = dt.year;
  const month = String(dt.month).padStart(2, '0');
  const day = String(dt.day).padStart(2, '0');
  const hour = String(dt.hour).padStart(2, '0');
  const minute = String(dt.minute).padStart(2, '0');
  const second = String(dt.second).padStart(2, '0');
  return `${year}-${month}-${day} ${hour}:${minute}:${second}`;
}

export function wait(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Convert valid CSS color to hex color
export function standardizeColor(color: string): string {
  const ctx = document.createElement("canvas").getContext("2d");
  if (ctx === null) return "#FFF";
  ctx.fillStyle = color;
  return ctx.fillStyle;
}

export function fullProcessName(process: ProcessInfo): string {
  return process.group ? `${process.group}:${process.name}` : process.name;
}
