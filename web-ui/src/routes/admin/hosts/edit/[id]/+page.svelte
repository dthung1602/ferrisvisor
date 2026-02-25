<script lang="ts">
  import HostForm from "../../HostForm.svelte";
  import { page } from '$app/state';
  import type { NewHost, UpdateHost } from "$lib/api/host";
  import { api } from "$lib";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  const hostId = parseInt(page.params.id || "0");

  let defaultHostValue = $state({
    name: "",
    port: 0,
    username: "",
    password: ""
  })

  $effect(() => {
    api.host.get(hostId).then(host => {
      console.log("Got host", host);
      defaultHostValue = host;
    })
  })

  async function onSubmit(newHost: NewHost) {
    const updateHost: UpdateHost = { ...newHost, id: hostId };
    console.log("Edit host", updateHost);
    await api.host.update(updateHost);
    await goto(resolve("/admin/hosts"));
  }
</script>

<h1>Edit Host</h1>
<HostForm {onSubmit} {...defaultHostValue} />
