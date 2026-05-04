<script lang="ts">
  import type { Host } from "$lib/api/host";

  type Props = {
    logType: "stdout" | "stderr";
    host: Host;
  };

  let { logType, host }: Props = $props();
</script>

<div>{logType} from host {host.name}</div>