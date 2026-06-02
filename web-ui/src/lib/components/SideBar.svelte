<script lang="ts">
  import { ChevronRight, Layers, LayoutDashboard, LogOut, Rocket, Server, Settings, Users } from "@lucide/svelte";
  import { Navigation } from "@skeletonlabs/skeleton-svelte";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { localstorage } from "$lib";

  const navItems = [
    { label: "Dashboard", href: resolve("/dashboard"), icon: LayoutDashboard },
    { label: "Groups", href: resolve("/admin/groups"), icon: Layers },
    { label: "Hosts", href: resolve("/admin/hosts"), icon: Server },
    { label: "Users", href: resolve("/admin/users"), icon: Users }
  ];

  let defaultIsSidebarOpen = localstorage.get(localstorage.SIDEBAR_OPEN, true) as boolean;
  localstorage.set(localstorage.SIDEBAR_OPEN, defaultIsSidebarOpen);

  let isSidebarOpen = $state(defaultIsSidebarOpen);

  function toggleSidebar() {
    isSidebarOpen = !isSidebarOpen;
    localstorage.set(localstorage.SIDEBAR_OPEN, isSidebarOpen);
  }
</script>

<aside
  class="relative z-40 border-r border-surface-200/40 bg-white/85 shadow-md backdrop-blur-xl transition-all duration-300 dark:border-surface-800 dark:bg-surface-950/40 dark:shadow-none
        {isSidebarOpen ? 'w-70' : 'w-21'}"
>
  <Navigation layout="sidebar" class="h-full w-full border-none bg-transparent">
    <Navigation.Header class="pb-6">
      <div class="flex items-center gap-3 overflow-hidden">
        <div
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-linear-to-br from-primary-500 to-tertiary-500 shadow-lg shadow-primary-500/20"
        >
          <Rocket class="text-on-primary size-6" />
        </div>
        {#if isSidebarOpen}
          <div class="whitespace-nowrap transition-opacity duration-300">
            <h1 class="text-xl font-bold tracking-tighter text-primary-500 uppercase">Ferrisvisor</h1>
            <p class="text-[9px] font-medium tracking-widest text-surface-500 uppercase">Celestial Engineer</p>
          </div>
        {/if}
      </div>
    </Navigation.Header>

    <Navigation.Content class="py-4">
      <Navigation.Group class="space-y-2">
        <Navigation.Menu class="flex flex-col gap-2">
          {#each navItems as item (item.href)}
            {@const isActive = page.url.pathname.startsWith(item.href)}
            <Navigation.TriggerAnchor
              href={item.href}
              class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all active:scale-95
                {isActive
                ? 'border-r-2 border-primary-500 bg-primary-500/10 font-bold text-primary-500 shadow-sm'
                : 'text-surface-600-400 hover:bg-surface-400/10 hover:text-surface-800-200'}
                {isSidebarOpen ? '' : 'w-13'}"
              title={item.label}
            >
              <item.icon class="size-5 shrink-0" />
              {#if isSidebarOpen}
                <Navigation.TriggerText class="whitespace-nowrap transition-opacity duration-300">
                  {item.label}
                </Navigation.TriggerText>
              {/if}
            </Navigation.TriggerAnchor>
          {/each}
        </Navigation.Menu>

        <div class="pt-4 opacity-50 {isSidebarOpen ? '' : 'w-13'}">
          <div class="mx-4 border-t-2 border-surface-500/20"></div>
        </div>

        <Navigation.Menu>
          <Navigation.TriggerAnchor
            href={resolve("/settings")}
            class="flex items-center gap-3 rounded-xl px-4 py-3 text-surface-600-400 transition-all
                 hover:bg-surface-400/10 hover:text-surface-800-200 active:scale-95
                   {isSidebarOpen ? '' : 'w-13'}"
            title="Settings"
          >
            <Settings class="size-5 shrink-0" />
            {#if isSidebarOpen}
              <Navigation.TriggerText class="whitespace-nowrap transition-opacity duration-300">
                Settings
              </Navigation.TriggerText>
            {/if}
          </Navigation.TriggerAnchor>
        </Navigation.Menu>
      </Navigation.Group>
    </Navigation.Content>

    <Navigation.Footer>
      <Navigation.Menu>
        <Navigation.TriggerAnchor
          href={resolve("/logout")}
          class="flex items-center gap-3 rounded-xl px-4 py-3 text-surface-600-400 transition-all
                 hover:bg-error-500/10 hover:text-error-500 active:scale-95
                 {isSidebarOpen ? '' : 'w-13'}"
          title="Logout"
        >
          <LogOut class="size-5 shrink-0" />
          {#if isSidebarOpen}
            <Navigation.TriggerText class="whitespace-nowrap transition-opacity duration-300">
              Logout
            </Navigation.TriggerText>
          {/if}
        </Navigation.TriggerAnchor>
      </Navigation.Menu>
    </Navigation.Footer>
  </Navigation>

  <!-- Toggle Sidebar Button -->
  <button
    onclick={toggleSidebar}
    class="absolute top-1/2 -right-4 flex size-8 -translate-y-1/2 items-center justify-center rounded-full border border-surface-400/20 bg-white text-surface-300 shadow-xl hover:text-primary-500 dark:bg-surface-800"
  >
    <ChevronRight class="size-4 transition-transform duration-300 {isSidebarOpen ? 'rotate-180' : ''}" />
  </button>
</aside>
