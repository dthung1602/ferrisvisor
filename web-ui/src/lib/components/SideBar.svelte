<script lang="ts">
  import { ChevronRight, Layers, LayoutDashboard, LogOut, Rocket, Server, Settings, Users } from "@lucide/svelte";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";

  const navItems = [
    { label: "Dashboard", href: resolve("/dashboard"), icon: LayoutDashboard },
    { label: "Groups", href: resolve("/admin/groups"), icon: Layers },
    { label: "Hosts", href: resolve("/admin/hosts"), icon: Server },
    { label: "Users", href: resolve("/admin/users"), icon: Users }
  ];

  let isSidebarOpen = $state(true);

  function toggleSidebar() {
    isSidebarOpen = !isSidebarOpen;
  }
</script>

<aside
  class="relative z-40 flex flex-col border-r border-surface-500/10 bg-surface-50-950/40 backdrop-blur-xl transition-all duration-300
        {isSidebarOpen ? 'w-64' : 'w-21'}"
>
  <!-- Brand Identity (Collapsed version) -->
  <div class="p-4">
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
  </div>

  <!-- Navigation Links -->
  <nav class="flex-1 space-y-2 px-4 py-4">
    {#each navItems as item (item.href)}
      {@const isActive = page.url.pathname.startsWith(item.href)}
      <a
        href={item.href}
        class="flex items-center gap-3 rounded-xl px-4 py-3 transition-all active:scale-95
          {isActive
          ? 'border-r-2 border-primary-500 bg-primary-500/10 font-bold text-primary-500 shadow-sm'
          : 'text-surface-600-400 hover:bg-surface-500/10 hover:text-surface-900-100'}"
        title={item.label}
      >
        <item.icon class="size-5 shrink-0" />
        {#if isSidebarOpen}
          <span class="whitespace-nowrap transition-opacity duration-300">{item.label}</span>
        {/if}
      </a>
    {/each}

    <div class="pt-4 opacity-50">
      <div class="mx-4 border-t border-surface-500/20"></div>
    </div>

    <a
      href="#"
      class="flex items-center gap-3 rounded-xl px-4 py-3 text-surface-600-400 transition-all hover:bg-surface-500/10 hover:text-surface-900-100 active:scale-95"
    >
      <Settings class="size-5 shrink-0" />
      {#if isSidebarOpen}
        <span class="whitespace-nowrap transition-opacity duration-300">Settings</span>
      {/if}
    </a>
  </nav>

  <!-- Sidebar Footer -->
  <div class="mt-auto p-4">
    <a
      href={resolve("/logout")}
      class="flex items-center gap-3 rounded-xl px-4 py-3 text-surface-600-400 transition-all hover:bg-error-500/10 hover:text-error-500 active:scale-95"
    >
      <LogOut class="size-5 shrink-0" />
      {#if isSidebarOpen}
        <span class="whitespace-nowrap transition-opacity duration-300">Logout</span>
      {/if}
    </a>
  </div>

  <!-- Toggle Sidebar Button -->
  <button
    onclick={toggleSidebar}
    class="absolute top-1/2 -right-3 flex size-6 -translate-y-1/2 items-center justify-center rounded-full border border-surface-500/20 bg-surface-100-900 text-surface-500 shadow-md hover:text-primary-500"
  >
    <ChevronRight class="size-3 transition-transform duration-300 {isSidebarOpen ? 'rotate-180' : ''}" />
  </button>
</aside>
