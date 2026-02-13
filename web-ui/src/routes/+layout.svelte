<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import logo from '$lib/assets/Original_Ferris.svg'
    import {resolve} from '$app/paths';

    let {children} = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>

<style>
    img {
        max-width: 4rem;
    }
</style>

<header class="z-50 sticky top-0 border-b border-surface-200-800 bg-surface-50-950/50 backdrop-blur-lg h-(--header-height) flex items-center">
    <a href="{resolve('/')}" class="container mx-auto px-4 flex items-center justify-between gap-2">
        <img src="{logo}" alt="Logo" class="m-2"/>
    </a>
</header>
<main>
    {@render children()}
</main>
