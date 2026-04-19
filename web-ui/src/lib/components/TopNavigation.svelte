<script lang="ts">
  import { Bell, Moon } from "@lucide/svelte";
  import { Avatar } from "@skeletonlabs/skeleton-svelte";

  import { getGlobalContext } from "$lib/global-state";

  let globalContext = getGlobalContext();
  let isLoggedIn = $derived(!globalContext.isLoadingCurrentUser && globalContext.currentUser);
  console.log("isLoggedIn", isLoggedIn, "globalContext", globalContext);
</script>

<header
  class="flex h-16 items-center justify-end border-b border-surface-500/10 bg-surface-50-950/20 px-8 backdrop-blur-md"
>
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
        <span class="block">
          {#if isLoggedIn}
            {globalContext.currentUser?.email}
          {:else}
            Anonymous
          {/if}
        </span>
      </span>
      <Avatar class="size-10 rounded-full border border-primary-500/30">
        <Avatar.Fallback>
          {#if isLoggedIn}
            {globalContext.currentUser?.email.slice(0, 2).toUpperCase()}
          {:else}
            ?
          {/if}
        </Avatar.Fallback>
      </Avatar>
    </div>
  </div>
</header>
