<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { api } from "$lib";

  import type { Permission } from "$lib/api/permission";

  const userId = parseInt(page.params.id || "0");
  let permissions: Permission[] = $state([]);

  console.log("PEERMISION FORM: ", userId, permissions);

  function fetchPerms() {
    api.permission.list(userId).then((values) => {
      permissions = values;
    });
  }

  $effect(fetchPerms);

  function goToEditPage(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const permId = target.dataset.id;
    goto(resolve(`/admin/users/edit/${userId}/permission/${permId}`));
  }

  function removeEntity(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const permId = parseInt(target.dataset.id as string);
    if (confirm(`Are you sure you want to remove permission with id ${permId} ?`)) {
      api.permission.remove(userId, permId).then(fetchPerms);
    }
  }
</script>

<p>Permissions</p>

<div class="table-wrap">
  <table class="table">
    <thead>
      <tr>
        <th>Id</th>
        <th>Group</th>
        <th>Host</th>
        <th>Service</th>
        <th>Can view</th>
        <th>Can act</th>
        <th></th>
      </tr>
    </thead>
    <tbody class="[&>tr]:hover:preset-tonal-primary">
      {#each permissions as perm (perm.id)}
        <tr>
          <td>{perm.id}</td>
          <td>{perm.group_name}</td>
          <td>{perm.host_name}</td>
          <td>{perm.service_name}</td>
          <td>{perm.can_view}</td>
          <td>{perm.can_act}</td>
          <td class="text-right">
            <button data-id={perm.id} onclick={goToEditPage} type="button" class="btn preset-filled-secondary-500">
              Edit
            </button>
            <button data-id={perm.id} onclick={removeEntity} type="button" class="btn preset-filled-error-500">
              Delete
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
