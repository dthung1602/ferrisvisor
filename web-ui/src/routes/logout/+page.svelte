<script lang="ts">
  import { api, cookies } from "$lib";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { Progress } from "@skeletonlabs/skeleton-svelte";

  api.auth.logout().then(() => {
    cookies.unsetSessionToken();
    goto(resolve("/"));
  });
</script>

<Progress class="w-fit items-center" value={null}>
  <Progress.Circle>
    <Progress.CircleTrack />
    <Progress.CircleRange />
  </Progress.Circle>
  <Progress.ValueText />
</Progress>
