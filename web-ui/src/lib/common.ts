export function formatDate(date: string) {
  if (!date) return "N/A";
  return new Date(date).toLocaleString();
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
