<script lang="ts">
  import { api } from "$lib";

  import type { Host } from "$lib/api/host";
  import type { ProcessConfig, ProcessInfo } from "$lib/api/process";
  import { fullProcessName } from "$lib/common";
  import { toaster } from "$lib/toaster";

  type Prop = {
    host: Host;
    process: ProcessInfo;
  };

  let { process, host }: Prop = $props();
  let isLoading = $state(true);
  let config: ProcessConfig | null = $state(null);

  $effect(() => {
    api.process
      .getConfigs(host.id, fullProcessName(process))
      .then((configs) => {
        if (configs.length === 0) {
          toaster.error({
            title: "No config found",
            description: `No config found for ${fullProcessName(process)} on ${host.name}`
          });
          return;
        }
        config = configs[0].config;
      })
      .catch((err) => {
        console.error(err);
        toaster.error({
          title: "Error loading config",
          description: `Error loading config for ${fullProcessName(process)} on ${host.name}`
        });
      })
      .finally(() => (isLoading = false));
  });
</script>

{#if isLoading}
  <div class="text-center">Loading...</div>
{:else if config}
  <table class="table">
    <tbody>
      {#each Object.entries(config) as [key, value] (key)}
        <tr>
          <td>{key}</td>
          <td>{value}</td>
        </tr>
      {/each}
    </tbody>
  </table>
{/if}
