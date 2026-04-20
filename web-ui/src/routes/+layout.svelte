<script lang="ts">
  import "./layout.css";

  import type { Snippet } from "svelte";

  import type { CurrentUser } from "$lib/api/auth";
  import favicon from "$lib/assets/favicon.svg";
  import Footer from "$lib/components/Footer.svelte";
  import SideBar from "$lib/components/SideBar.svelte";
  import TopNavigation from "$lib/components/TopNavigation.svelte";
  import { setGlobalContext, type GlobalState } from "$lib/global-state";

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
      localStorage.getItem("theme") === "dark" ||
      (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches);
  });

  $effect(() => {
    if (globalContext.isDarkMode) {
      document.documentElement.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

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
