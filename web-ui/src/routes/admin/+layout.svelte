<script lang="ts">
  import ProtectedRoute from "$lib/components/ProtectedRoute.svelte";

  const { children } = $props();
</script>

<ProtectedRoute>
  {@render children()}
</ProtectedRoute>
