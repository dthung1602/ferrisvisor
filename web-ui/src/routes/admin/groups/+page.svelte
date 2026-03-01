<script lang="ts">
  import { Server } from "lucide-svelte";
  import { api } from "$lib";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import type { Host } from "$lib/api/host";

  let hosts: Host[] = $state([]);

  $effect(() => {
    api.host.list().then((fetchedHosts: Host[]) => {
      hosts = fetchedHosts;
    });
  });

  function goToNewHostPage() {
    goto(resolve("/admin/hosts/new"));
  }

  function goToEditHostPage(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const hostId = target.dataset.id;
    goto(resolve(`/admin/hosts/edit/${hostId}`));
  }
</script>

<div class="flex justify-between">
  <h1>Hosts</h1>
  <button onclick={goToNewHostPage} type="button" class="btn preset-filled-success-500">
    <Server size={18} />
    Add host
  </button>
</div>

<div class="table-wrap">
  <table class="table">
    <tbody class="[&>tr]:hover:preset-tonal-primary">
    {#each hosts as host (host.id)}
      <tr>
        <td>{host.id}</td>
        <td>{host.name}</td>
        <td>{host.port}</td>
        <td class="text-right">
          <button data-id={host.id} onclick={goToEditHostPage} type="button" class="btn preset-filled-secondary-500">Edit</button>
          <button type="button" class="btn preset-filled-error-500">Delete</button>
        </td>
      </tr>
    {/each}
    </tbody>
  </table>
</div>
