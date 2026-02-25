<script lang="ts">
  import "./layout.css";
  import favicon from "$lib/assets/favicon.svg";
  import logo from "$lib/assets/Original_Ferris.svg";
  import { resolve } from "$app/paths";

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<header
  class="sticky top-0 z-50 flex h-(--header-height) items-center border-b border-surface-200-800 bg-surface-50-950/50 backdrop-blur-lg"
>
  <a href={resolve("/")} class="container mx-auto flex items-center justify-between gap-2 px-4">
    <img src={logo} alt="Logo" class="m-2" />
  </a>
</header>
<main>
  {@render children()}
</main>

<style>
  img {
    max-width: 4rem;
  }
</style>
