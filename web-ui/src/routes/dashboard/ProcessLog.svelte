<script lang="ts">
  import { untrack } from "svelte";

  import { api } from "$lib";

  import type { Host } from "$lib/api/host";
  import type { ProcessInfo, ProcessLogType } from "$lib/api/process";
  import { fullProcessName } from "$lib/common";
  import { toaster } from "$lib/toaster";

  const LOG_CHUNK_SIZE = 2048;
  const LOG_TAIL_INTERVAL = 5_000;
  const MAX_LOG_SIZE = 65_536; // must > LOG_CHUNK_SIZE

  type Props = {
    logType: ProcessLogType;
    host: Host;
    process: ProcessInfo;
  };

  let timeoutHandler: number | null = null;
  let offset = 0;
  let logText = $state("");
  let isDestroyed = false;
  let autoscroll = $state(true);
  let bottomEl: HTMLDivElement | null = $state(null);

  $effect(() => {
    if (autoscroll && bottomEl && logText !== undefined) {
      bottomEl.scrollIntoView({ behavior: "auto", block: "end" });
    }
  });

  async function fetchLogChunk() {
    if (isDestroyed) {
      if (timeoutHandler !== null) {
        window.clearTimeout(timeoutHandler);
      }
      return
    }

    let logResp;
    try {
      logResp = await api.process.tailLog(logType, host.id, fullProcessName(process), offset, LOG_CHUNK_SIZE);
    } catch (e) {
      console.error(e);
      toaster.error({ title: "Failed to get log", description: "Error: " + e });
      if (!isDestroyed) {
        timeoutHandler = window.setTimeout(fetchLogChunk, LOG_TAIL_INTERVAL);
      }
      return;
    }

    // if overflow:
    // - show the tail of the log
    // - stop the interval
    // - increase the offset
    // - fetch the next chunk immediately to cactch up with the new tail
    if (logResp.overflow) {
      offset = logResp.offset;
      logText = logResp.log;
      if (timeoutHandler !== null) {
        window.clearTimeout(timeoutHandler);
        timeoutHandler = null;
      }
      if (!isDestroyed) {
        fetchLogChunk();
      }
      return;
    }

    // otherwise:
    // - append the new chunk to the log (but no longer than MAX_LOG_SIZE)
    // - set the new offset for the next fetch
    // - start the interval again
    if (logResp.offset > offset) {
      const delta = logResp.offset - offset;
      const newLogText = logResp.log.substring(logResp.log.length - delta);
      logText = logText.substring(logText.length + newLogText.length - MAX_LOG_SIZE) + newLogText;
      offset = logResp.offset;
    }
    if (!isDestroyed) {
      timeoutHandler = window.setTimeout(fetchLogChunk, LOG_TAIL_INTERVAL);
    }
  }

  $effect(() => {
    untrack(() => {
      fetchLogChunk();
    });
    return () => {
      isDestroyed = true;
      if (timeoutHandler !== null) {
        window.clearTimeout(timeoutHandler);
      }
    };
  });

  let { logType, host, process }: Props = $props();
</script>

<div
  class="sticky top-[51px] z-10 flex items-center justify-between border-b border-surface-500/10 bg-surface-100-900 py-2 px-4"
>
  <label class="flex cursor-pointer items-center justify-end gap-2 text-sm select-none w-full">
    <input type="checkbox" bind:checked={autoscroll} class="checkbox" />
    <span class="text-xs font-medium">Autoscroll</span>
  </label>
</div>

<div class="p-2 pb-16">
  <pre>{logText}</pre>
  <div bind:this={bottomEl}></div>
</div>
