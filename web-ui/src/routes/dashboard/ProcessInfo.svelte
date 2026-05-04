<script lang="ts">
  import type { ProcessInfo } from "$lib/api/process";

  type Prop = {
    process: ProcessInfo;
  };

  let { process }: Prop = $props();
</script>

<table class="table">
  <tbody>
    {#each Object.entries(process) as [key, value] (key)}
      <tr>
        <td>{key}</td>
        <td>{value}</td>
      </tr>
    {/each}
  </tbody>
</table>
