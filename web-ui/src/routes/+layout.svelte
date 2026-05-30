<script lang="ts">
  import "./layout.css";

  import type { Snippet } from "svelte";

  import { X } from "@lucide/svelte";
  import { Toast } from "@skeletonlabs/skeleton-svelte";
  import { localstorage } from "$lib";

  import type { CurrentUser } from "$lib/api/auth";
  import favicon from "$lib/assets/favicon.svg";
  import Footer from "$lib/components/Footer.svelte";
  import SideBar from "$lib/components/SideBar.svelte";
  import TopNavigation from "$lib/components/TopNavigation.svelte";
  import { setGlobalContext, type GlobalState } from "$lib/global-state";
  import { toaster } from "$lib/toaster";

  type Props = {
    children: Snippet<[]>;
    data: { currentUser: CurrentUser | null };
  };
  let { children, data }: Props = $props();

  const globalContext = $state({
    currentUser: data.currentUser,
    isDarkMode: false
  } as GlobalState);
  setGlobalContext(globalContext);

  $effect(() => {
    globalContext.isDarkMode =
      localstorage.get(localstorage.THEME) === "dark" ||
      (!localstorage.contain(localstorage.THEME) && window.matchMedia("(prefers-color-scheme: dark)").matches);
  });

  $effect(() => {
    if (globalContext.isDarkMode) {
      document.documentElement.classList.add("dark");
      localstorage.set(localstorage.THEME, "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localstorage.set(localstorage.THEME, "light");
    }
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<Toast.Group {toaster}>
  {#snippet children(toast)}
    <Toast
      {toast}
      class="flex min-w-80 items-center justify-between gap-4 rounded-xl border p-4 shadow-2xl backdrop-blur-xl"
    >
      <Toast.Message class="flex-1 space-y-1">
        {#if toast.title}
          <Toast.Title class="text-sm font-bold tracking-tight">{toast.title}</Toast.Title>
        {/if}
        {#if toast.description}
          <Toast.Description class="text-xs opacity-70">{toast.description}</Toast.Description>
        {/if}
      </Toast.Message>
      <Toast.CloseTrigger class="rounded-full p-1 transition-colors">
        <X class="size-4" />
      </Toast.CloseTrigger>
    </Toast>
  {/snippet}
</Toast.Group>

<main>
  <div class="flex h-screen overflow-hidden bg-surface-100-900">
    <!-- Side Navigation -->
    <SideBar />

    <!-- Main Content Area -->
    <div class="flex flex-1 flex-col overflow-hidden">
      <!-- Top App Bar -->
      <TopNavigation />

      <!-- Content Section -->
      <main class="flex-1 overflow-y-auto p-8">
        <div class="container mx-auto">
          {@render children()}
        </div>
      </main>

      <!-- Status Footer -->
      <Footer />
    </div>
  </div>
</main>
