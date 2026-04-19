<script lang="ts">
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import {
    LayoutDashboard,
    Group,
    Users,
    Server,
    Settings,
    LogOut,
    Bell,
    Moon,
    Rocket,
    ChevronRight
  } from "@lucide/svelte";
  import { Avatar } from "@skeletonlabs/skeleton-svelte";
  import logo from "$lib/assets/Original_Ferris.svg";
  import { GITHUB_REPO_URL, VERSION } from "$lib/constants";

  let { children } = $props();

  const navItems = [
    { label: "Dashboard", href: resolve("/dashboard"), icon: LayoutDashboard },
    { label: "Groups", href: resolve("/admin/groups"), icon: Group },
    { label: "Hosts", href: resolve("/admin/hosts"), icon: Server },
    { label: "Users", href: resolve("/admin/users"), icon: Users }
  ];

  // const adminSubNav = [
  //   { label: "Overview", href: "#" },
  //   { label: "Logs", href: "#" },
  //   { label: "Metrics", href: "#" },
  //   { label: "Audit", href: "#" }
  // ];

  let isSidebarOpen = $state(true);

  function toggleSidebar() {
    isSidebarOpen = !isSidebarOpen;
  }
</script>

<div class="flex h-screen overflow-hidden bg-surface-100-900">
  <!-- Side Navigation -->
  <aside
    class="relative z-40 flex flex-col border-r border-surface-500/10 bg-surface-50-950/40 backdrop-blur-xl transition-all duration-300 {isSidebarOpen
      ? 'w-64'
      : 'w-21'}"
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
          rel="external"
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

  <!-- Main Content Area -->
  <div class="flex flex-1 flex-col overflow-hidden">
    <!-- Top App Bar -->
    <header
      class="flex h-16 items-center justify-end border-b border-surface-500/10 bg-surface-50-950/20 px-8 backdrop-blur-md"
    >
      <!--      <div class="flex items-center gap-8">-->
      <!--        <div class="flex items-center gap-6 text-[10px] font-bold tracking-widest uppercase">-->
      <!--          {#each adminSubNav as item}-->
      <!--            <a href={item.href} class="transition-colors hover:text-primary-500 {item.label === 'Overview' ? 'text-primary-500 border-b border-primary-500 pb-1' : 'text-surface-500'}">-->
      <!--              {item.label}-->
      <!--            </a>-->
      <!--          {/each}-->
      <!--        </div>-->

      <!--        <div class="relative group hidden lg:block">-->
      <!--          <Search class="absolute left-3 top-1/2 size-3.5 -translate-y-1/2 text-surface-500" />-->
      <!--          <input-->
      <!--            type="text"-->
      <!--            placeholder="GLOBAL TELEMETRY SEARCH..."-->
      <!--            class="input bg-surface-500/10 border-none rounded-full py-1.5 pl-10 pr-4 text-[9px] w-64 focus:ring-2 focus:ring-primary-500/20 font-bold tracking-widest"-->
      <!--          />-->
      <!--        </div>-->
      <!--      </div>-->

      <div class="flex items-center gap-3">
        <button class="btn-icon btn-icon-lg transition-colors hover:text-primary-500">
          <Bell class="size-6" />
        </button>
        <button class="btn-icon btn-icon-lg transition-colors hover:text-primary-500">
          <Moon class="size-6" />
        </button>
        <div class="mx-2 h-4 border-l border-surface-500/20"></div>
        <div class="flex items-center gap-2">
          <span class="hidden text-right text-[10px] font-bold tracking-widest uppercase sm:block">
            <span class="block">Node-Alpha-01</span>
            <span class="block opacity-40">System Active</span>
          </span>
          <Avatar class="size-10 rounded-full border border-primary-500/30">
            <Avatar.Image src={logo} alt="base" />
            <Avatar.Fallback>SK</Avatar.Fallback>
          </Avatar>
        </div>
      </div>
    </header>

    <!-- Content Section -->
    <main class="flex-1 overflow-y-auto p-8">
      <div class="container mx-auto">
        {@render children()}
      </div>
    </main>

    <!-- Status Footer -->
    <footer
      class="flex items-center justify-between border-t border-surface-500/10 bg-surface-50-950 px-8 py-2 text-[9px] font-bold tracking-widest text-surface-500 uppercase"
    >
      <div class="flex items-center gap-6">
        <span class="text-primary-500">version {VERSION}</span>
      </div>
      <div class="flex items-center gap-6">
        <a href={resolve("/about")}>About</a>
        <a href={GITHUB_REPO_URL} rel="external" target="_blank">Github</a>
      </div>
    </footer>
  </div>
</div>
