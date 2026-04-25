<script lang="ts">
  import type { Snippet } from "svelte";

  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  import { getGlobalContext } from "$lib/global-state";
  import { resolve } from "$app/paths";

  type Prop = {
    children: Snippet<[]>;
    requiredAdmin?: boolean;
  };

  let { children, requiredAdmin = false }: Prop = $props();

  let globalContext = getGlobalContext();

  $effect(() => {
    if (globalContext.currentUser === null) {
      const redirectTo = page.url.pathname;
      // eslint-disable-next-line svelte/no-navigation-without-resolve
      goto(`/login?redirect=${redirectTo}`);
    } else if (requiredAdmin && !globalContext.currentUser.is_admin) {
      // eslint-disable-next-line svelte/no-navigation-without-resolve
      goto("/error?code=403");
    }
  });
</script>

{#if globalContext.currentUser === null || (requiredAdmin && !globalContext.currentUser.is_admin)}
  <div class="spinner">Redirecting...</div>
{:else}
  {@render children()}
{/if}
