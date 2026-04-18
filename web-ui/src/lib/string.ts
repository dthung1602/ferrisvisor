import { startCase } from "lodash";

export function normalizedFieldNameToDisplay(fieldName: string): string {
  const parts = startCase(fieldName).split(" ");
  if (parts[parts.length - 1] === "Id") {
    parts.pop();
  }
  return parts.join(" ");
}

export function formatDate(date: string) {
  if (!date) return "N/A";
  return new Date(date).toLocaleString();
}
