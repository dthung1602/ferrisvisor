<script lang="ts">
  import ProtectedRoute from "$lib/components/ProtectedRoute.svelte";

  const { children } = $props();
</script>

<ProtectedRoute requiredAdmin={true}>
  {@render children()}
</ProtectedRoute>
