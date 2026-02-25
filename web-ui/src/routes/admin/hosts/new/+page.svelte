<script lang="ts">
  import HostForm from "../HostForm.svelte";
  import type { NewHost } from "$lib/api/host";
  import { api } from "$lib";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  async function onSubmit(newHost: NewHost) {
    console.log("New host", newHost);
    await api.host.create(newHost);
    await goto(resolve("/admin/hosts"));
  }
</script>

<h1>New Host</h1>
<HostForm {onSubmit} />
