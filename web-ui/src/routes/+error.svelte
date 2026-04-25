<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  // eslint-disable-next-line svelte/no-navigation-without-resolve
  goto(`/error?code=${page.status}`);
</script>
