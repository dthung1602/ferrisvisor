export function formatDate(date: string) {
  if (!date) return "N/A";
  return new Date(date).toLocaleString();
}

export function wait(ms: number): Promise<void> {
  return new Promise(resolve =>
    setTimeout(resolve, ms)
  )
}
