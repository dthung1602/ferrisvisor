<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  import { group, host } from "$lib/api";
  import type { Group } from "$lib/api/group";
  import type { NewHost } from "$lib/api/host";
  import { toaster } from "$lib/toaster";

  import HostForm from "../HostForm.svelte";

  let groups: Group[] = $state([]);
  let formData = $state({
    group_id: 0,
    name: "",
    hostname: "",
    port: 22,
    username: "",
    password: ""
  } as NewHost);

  async function fetchInitData() {
    try {
      groups = await group.list();
      if (groups.length > 0) {
        formData.group_id = groups[0].id;
      }
    } catch (e) {
      console.error("Failed to fetch groups", e);
    }
  }

  $effect(() => {
    fetchInitData();
  });

  async function handleSave() {
    try {
      await host.create(formData);
      toaster.success({ title: "Host Created", description: "The new host has been added to the registry." });
      await goto(resolve("/admin/hosts"));
    } catch (e) {
      console.error(e);
      toaster.error({
        title: "Creation Failed",
        description: "Could not create the new host. Please check the details."
      });
    }
  }

  function handleDiscard() {
    goto(resolve("/admin/hosts"));
  }
</script>

<div class="mx-auto max-w-2xl space-y-8 px-4 py-12">
  <HostForm bind:host={formData} {groups} isEdit={false} onSave={handleSave} onDiscard={handleDiscard} />
</div>
