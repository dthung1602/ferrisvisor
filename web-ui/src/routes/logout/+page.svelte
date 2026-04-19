<script lang="ts">
  import { Progress } from "@skeletonlabs/skeleton-svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { api, cookies } from "$lib";

  import { getGlobalContext } from "$lib/global-state";

  const globalContext = getGlobalContext();

  api.auth.logout().then(() => {
    cookies.unsetSessionToken();
    globalContext.currentUser = null;
    goto(resolve("/login"));
  });
</script>

<Progress class="w-fit items-center" value={null}>
  <Progress.Circle>
    <Progress.CircleTrack />
    <Progress.CircleRange />
  </Progress.Circle>
  <Progress.ValueText />
</Progress>
