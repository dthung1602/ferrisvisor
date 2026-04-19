<script lang="ts">
  import "./layout.css";

  import { api } from "$lib";

  import favicon from "$lib/assets/favicon.svg";
  import Footer from "$lib/components/Footer.svelte";
  import SideBar from "$lib/components/SideBar.svelte";
  import TopNavigation from "$lib/components/TopNavigation.svelte";
  import { setGlobalContext, type GlobalState } from "$lib/global-state";

  let { children } = $props();

  $effect(() => {
    api.auth
      .me()
      .then((currentUser) => {
        console.log("Current user:", currentUser);
        globalContext.currentUser = currentUser;
      })
      .finally(() => {
        globalContext.isLoadingCurrentUser = false;
      });
  });

  const globalContext = $state({
    currentUser: null,
    isLoadingCurrentUser: true
  } as GlobalState);
  setGlobalContext(globalContext);
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
