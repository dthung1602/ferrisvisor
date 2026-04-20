<script>
  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  import { getGlobalContext } from "$lib/global-state";

  let { children } = $props();

  let globalContext = getGlobalContext();

  $effect(() => {
    if (globalContext.currentUser === null) {
      const redirectTo = page.url.pathname;
      // eslint-disable-next-line svelte/no-navigation-without-resolve
      goto(`/login?redirect=${redirectTo}`);
    }
  });
</script>

{#if globalContext.currentUser === null}
  <div class="spinner">Redirecting...</div>
{:else}
  {@render children()}
{/if}
