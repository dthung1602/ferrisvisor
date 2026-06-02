<script lang="ts">
  import { localstorage } from "$lib";

  import TimezoneSelector from "$lib/components/TimezoneSelector.svelte";

  let selectedTimezone = $state(localstorage.get(localstorage.TIMEZONE, "UTC"));
  let dashboardRefreshRate = $state(localstorage.get(localstorage.REFRESH_RATE, 15));

  $effect(() => {
    localstorage.set(localstorage.TIMEZONE, selectedTimezone);
  });

  $effect(() => {
    localstorage.set(localstorage.REFRESH_RATE, dashboardRefreshRate);
  });
</script>

<div class="space-y-8">
  <!-- Header Section -->
  <div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
    <div>
      <h2 class="text-4xl font-black tracking-tighter uppercase">Settings</h2>
      <p class="mt-1 font-medium tracking-wide text-secondary-700-300">All settings</p>
    </div>
  </div>

  <div class="grid grid-cols-12 gap-8">
    <!-- Hosts Registry (Bento-style list) -->
    <div
      class="relative col-span-12 space-y-4 overflow-hidden card rounded-xl border border-surface-200/30 bg-white p-6 shadow-xl backdrop-blur-xl lg:col-span-7 dark:border-surface-800 dark:bg-surface-900 dark:shadow-none"
    >
      <div class="flex items-center justify-between space-y-1.5">
        <label for="refresh_rate" class="ml-1 font-bold opacity-70">Dashboard refresh rate (sec)</label>
        <input
          class="input max-w-32 rounded-xl border border-surface-200 bg-white px-4 py-3 text-sm shadow-inner focus:ring-2 focus:ring-primary-500/80 dark:border-surface-700 dark:bg-surface-900"
          type="number"
          name="refresh_rate"
          required
          bind:value={dashboardRefreshRate}
          min="1"
        />
      </div>
      <div class="flex items-center justify-between space-y-1.5">
        <label for="timezone" class="ml-1 font-bold opacity-70">Timezone</label>
        <TimezoneSelector bind:selectedTimezone />
      </div>
    </div>
  </div>
</div>
