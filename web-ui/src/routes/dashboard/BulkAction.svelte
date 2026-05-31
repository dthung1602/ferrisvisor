<script lang="ts">
  import { ChevronsDownUp, ChevronsUpDown, ListFilter, Play, RotateCcw, Square, X } from "@lucide/svelte";
  import { Menu, Portal, Progress } from "@skeletonlabs/skeleton-svelte";
  import { api } from "$lib";

  import type { ProcessAction, ProcessActionRequest, ProcessInfo } from "$lib/api/process";
  import type { ProcessState } from "$lib/constants";
  import { toaster } from "$lib/toaster";

  import { STATE_ACTION_MAP } from "./common.ts";

  type Props = {
    selectedProcesses: ProcessActionRequest[];
    processInfoByHost: Map<number, ProcessInfo[]>;
    refreshAllProcessInfo: () => void;
    setAllHostPanelCollapseState: (collapsed: boolean) => void;
  };

  let {
    selectedProcesses = $bindable([]),
    processInfoByHost,
    refreshAllProcessInfo,
    setAllHostPanelCollapseState
  }: Props = $props();

  let actionInProgress = $state(false);

  function getProcessState(hostId: number, processName: string): ProcessState | null {
    const hostProcesses = processInfoByHost.get(hostId) ?? [];
    const p = hostProcesses.find(
      (proc) => proc.name === processName || (proc.group ? `${proc.group}:${proc.name}` : proc.name) === processName
    );
    return p ? p.statename : null;
  }

  async function executeBulkAction(action: ProcessAction) {
    if (selectedProcesses.length === 0) {
      return;
    }

    actionInProgress = true;

    const reqs: ProcessActionRequest[] = [];

    for (const item of selectedProcesses) {
      const state = getProcessState(item.host_id, item.process_name);
      if (!state) continue;

      if (action === "restart") {
        reqs.push(item);
      } else if (action === "start" && STATE_ACTION_MAP[state] === "start") {
        reqs.push(item);
      } else if (action === "stop" && STATE_ACTION_MAP[state] === "stop") {
        reqs.push(item);
      }
    }

    if (reqs.length === 0) {
      toaster.warning({
        title: "No applicable processes",
        description: `None of the selected processes are in a state that can be ${action}ed.`
      });
      return;
    }

    try {
      const respPromise = api.process.action(action, reqs);
      let actionResponseReceived = false;

      const timer = setTimeout(() => {
        if (!actionResponseReceived) {
          refreshAllProcessInfo();
        }
      }, 200);

      const resp = await respPromise;
      actionResponseReceived = true;
      clearTimeout(timer);

      const errors = resp
        .filter((r) => !r.success)
        .map((r) => `Host ${r.host_id}: ${r.process_name} - ${r.error}`)
        .join("\n");

      if (errors) {
        toaster.error({
          title: `Bulk ${action} partial failure`,
          description: errors
        });
      } else {
        toaster.success({
          title: `Bulk ${action} successful`,
          description: `All ${reqs.length} processes ${action}ed successfully.`
        });
      }
      refreshAllProcessInfo();
    } catch (e) {
      console.error(`Failed to execute bulk ${action}`, e);
      toaster.error({
        title: "System Error",
        description: `An unexpected error occurred while executing bulk ${action}.`
      });
    } finally {
      actionInProgress = false;
    }
  }

  function handleCollapseAll() {
    setAllHostPanelCollapseState(true);
  }

  function handleExpandAll() {
    setAllHostPanelCollapseState(false);
  }

  function handleStart() {
    executeBulkAction("start");
  }

  function handleRestart() {
    executeBulkAction("restart");
  }

  function handleStop() {
    executeBulkAction("stop");
  }

  function handleUnselect() {
    selectedProcesses = [];
  }

  const ACTIONS = {
    collapse: handleCollapseAll,
    expand: handleExpandAll,
    start: handleStart,
    restart: handleRestart,
    stop: handleStop,
    unselect: handleUnselect
  };
  function handleSelectAction(detail: { value: string }) {
    ACTIONS[detail.value as keyof typeof ACTIONS]();
  }
</script>

<Menu positioning={{ placement: "bottom-end", gutter: 8, sameWidth: false }} onSelect={handleSelectAction}>
  <Menu.Trigger
    disabled={actionInProgress}
    class="flex items-center justify-center gap-2 rounded-xl border-2 border-surface-200 bg-white px-4 py-3 text-sm font-medium transition-all hover:bg-surface-100/20 active:scale-[0.99] disabled:cursor-progress disabled:opacity-50 md:col-span-2 lg:col-auto lg:ml-auto lg:shrink-0 dark:border-surface-700 dark:bg-surface-900 dark:hover:bg-surface-500/40"
  >
    {#if actionInProgress}
      <Progress class="w-fit items-center" value={null}>
        <Progress.Circle class="[--size:--spacing(4)]">
          <Progress.CircleTrack />
          <Progress.CircleRange />
        </Progress.Circle>
      </Progress>
    {:else}
      <ListFilter class="size-4" />
    {/if}

    <span>Bulk Action {selectedProcesses.length > 0 ? `(${selectedProcesses.length})` : ""}</span>
  </Menu.Trigger>
  <Portal>
    <Menu.Positioner>
      <Menu.Content
        class="z-100 overflow-hidden rounded-xl border-2 border-surface-200 bg-white shadow-xl backdrop-blur-xl dark:border-surface-700 dark:bg-surface-900"
      >
        <Menu.Item
          value="collapse"
          class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
        >
          <Menu.ItemText class="flex items-center gap-2 text-sm font-medium">
            <ChevronsDownUp size="16" /> Collapse all hosts
          </Menu.ItemText>
        </Menu.Item>
        <Menu.Item
          value="expand"
          class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
        >
          <Menu.ItemText class="flex items-center gap-2 text-sm font-medium">
            <ChevronsUpDown size="16" /> Expand all hosts
          </Menu.ItemText>
        </Menu.Item>
        <Menu.Item
          disabled={selectedProcesses.length === 0}
          value="unselect"
          class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
        >
          <Menu.ItemText class="flex items-center gap-2 text-sm font-medium">
            <X size="16" /> Unselect everything
          </Menu.ItemText>
        </Menu.Item>
        <Menu.Separator />
        <Menu.Item
          disabled={selectedProcesses.length === 0}
          value="start"
          class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
        >
          <Menu.ItemText class="flex items-center gap-2 text-sm font-medium">
            <Play size="16" /> Start
          </Menu.ItemText>
        </Menu.Item>
        <Menu.Item
          disabled={selectedProcesses.length === 0}
          value="restart"
          class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
        >
          <Menu.ItemText class="flex items-center gap-2 text-sm font-medium">
            <RotateCcw size="16" /> Restart
          </Menu.ItemText>
        </Menu.Item>
        <Menu.Item
          disabled={selectedProcesses.length === 0}
          value="stop"
          class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
        >
          <Menu.ItemText class="flex items-center gap-2 text-sm font-medium">
            <Square size="16" /> Stop
          </Menu.ItemText>
        </Menu.Item>
      </Menu.Content>
    </Menu.Positioner>
  </Portal>
</Menu>
