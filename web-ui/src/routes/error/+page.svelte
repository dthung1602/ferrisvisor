<script lang="ts">
  import { Milestone } from "@lucide/svelte";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import FERRIS_IMG from "$lib/assets/Confused_Ferris.svg";
  const code = page.url.searchParams.get("code") ?? "???";

  const statusMap = new Map([
    [400, "Bad Request"],
    [401, "Unauthorized Access"],
    [403, "Forbidden"],
    [404, "Not Found"],
    [500, "Internal Server Error"]
  ]);
  const statusText = $derived(statusMap.get(Number(code)) ?? "Unknown Error");
</script>

<div class="relative flex flex-1 flex-col items-center justify-center px-6 text-center">
  <div class="relative z-10 w-full max-w-2xl">
    <!-- Bento Error Container -->
    <div class="items-stretch gap-6 md:grid-cols-12">
      <!-- Hero Section (Ferris) -->
      <div class="mb-24 flex flex-col items-center md:col-span-12">
        <div class="relative mb-6 h-64 w-64">
          <div class="bg-primary/20 absolute inset-0 scale-110 rounded-full blur-3xl"></div>
          <img
            alt="Ferris the Crab"
            class="relative z-10 h-full w-full object-contain drop-shadow-2xl filter"
            src={FERRIS_IMG}
          />
        </div>
        <h2 class="font-headline mb-2 text-6xl font-extrabold tracking-tighter text-white md:text-8xl">{code}</h2>
        <p class="font-headline text-primary text-2xl font-medium tracking-tight uppercase">
          {statusText}
        </p>
      </div>
      <!-- Detail Blocks (Asymmetric Bento) -->

      <button
        type="button"
        class="btn preset-filled-primary-500 px-4 py-2"
        onclick={() => goto(resolve("/dashboard"))}
      >
        Back to Dashboard
      </button>
    </div>
  </div>
</div>
