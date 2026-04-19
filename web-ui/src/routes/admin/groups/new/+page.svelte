<script lang="ts">
  import type { NewGroup } from "$lib/api/group";
  import { group } from "$lib/api";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import GroupForm from "../GroupForm.svelte";

  let formData = $state({
    name: "",
    description: ""
  } as NewGroup);

  async function handleSave() {
    try {
      await group.create(formData);
      await goto(resolve("/admin/groups"));
    } catch (e) {
      console.error(e);
    }
  }

  function handleDiscard() {
    goto(resolve("/admin/groups"));
  }
</script>

<div class="mx-auto max-w-2xl space-y-8 px-4 py-12">
  <GroupForm bind:group={formData} isEdit={false} onSave={handleSave} onDiscard={handleDiscard} />
</div>
