<script lang="ts">
  import { GripVerticalIcon, Logs, MaximizeIcon, MinimizeIcon, MinusIcon, XIcon } from "@lucide/svelte";
  import { FloatingPanel, Portal, Tabs } from "@skeletonlabs/skeleton-svelte";

  import type { Host } from "$lib/api/host";
  import type { ProcessInfo } from "$lib/api/process";

  import ProcessConfig from "./ProcessConfig.svelte";
  import ProcessInfoComponent from "./ProcessInfo.svelte";
  import ProcessLog from "./ProcessLog.svelte";

  type Prop = {
    host: Host;
    process: ProcessInfo;
  };

  type TabType = "info" | "config" | "stdout" | "stderr";

  let { process, host }: Prop = $props();

  let selectedTab: TabType = $state("info");

  let isOpened = $state(false);
  function setOpen(details: { open: boolean }) {
    isOpened = details.open;
  }
</script>

<FloatingPanel defaultSize={{ width: 600, height: 500 }} onOpenChange={setOpen}>
  <FloatingPanel.Trigger
    class="flex items-center justify-center gap-1.5 rounded-md p-1.5 px-2 transition-colors not-disabled:text-surface-900-100 not-disabled:hover:bg-surface-500/50 disabled:cursor-not-allowed disabled:text-surface-300"
  >
    <Logs size="16" /> Info
  </FloatingPanel.Trigger>

  {#if isOpened}
    <Portal>
      <FloatingPanel.Positioner class="z-50">
        <FloatingPanel.Content class="flex h-full flex-col overflow-hidden">
          <FloatingPanel.DragTrigger>
            <FloatingPanel.Header>
              <FloatingPanel.Title>
                <GripVerticalIcon class="size-4" />
                {host.name} - {process.name}
              </FloatingPanel.Title>
              <FloatingPanel.Control>
                <FloatingPanel.StageTrigger stage="minimized">
                  <MinusIcon class="size-4" />
                </FloatingPanel.StageTrigger>
                <FloatingPanel.StageTrigger stage="maximized">
                  <MaximizeIcon class="size-4" />
                </FloatingPanel.StageTrigger>
                <FloatingPanel.StageTrigger stage="default">
                  <MinimizeIcon class="size-4" />
                </FloatingPanel.StageTrigger>
                <FloatingPanel.CloseTrigger>
                  <XIcon className="size-4" />
                </FloatingPanel.CloseTrigger>
              </FloatingPanel.Control>
            </FloatingPanel.Header>
          </FloatingPanel.DragTrigger>

          <FloatingPanel.Body class="flex-1 overflow-y-auto pt-0">
            <Tabs value={selectedTab} onValueChange={(details) => (selectedTab = details.value as TabType)}>
              <Tabs.List class="sticky top-0 z-20 border-b border-surface-500/10 bg-surface-100-900 pt-2 mb-0">
                <Tabs.Trigger value="info">Info</Tabs.Trigger>
                <Tabs.Trigger value="config">Config</Tabs.Trigger>
                <Tabs.Trigger value="stdout">Stdout</Tabs.Trigger>
                <Tabs.Trigger value="stderr">Stderr</Tabs.Trigger>
                <Tabs.Indicator />
              </Tabs.List>
              <Tabs.Content value="info">
                <ProcessInfoComponent {process} />
              </Tabs.Content>
              <Tabs.Content value="config">
                {#if selectedTab === "config"}
                  <!-- Only render if it's selected-->
                  <ProcessConfig {host} {process} />
                {/if}
              </Tabs.Content>
              <Tabs.Content value="stdout">
                {#if selectedTab === "stdout"}
                  <!-- Only render if it's selected-->
                  <ProcessLog logType="stdout" {host} {process} />
                {/if}
              </Tabs.Content>
              <Tabs.Content value="stderr">
                {#if selectedTab === "stderr"}
                  <!-- Only render if it's selected-->
                  <ProcessLog logType="stderr" {host} {process} />
                {/if}
              </Tabs.Content>
            </Tabs>
          </FloatingPanel.Body>

          <FloatingPanel.ResizeTrigger axis="n" />
          <FloatingPanel.ResizeTrigger axis="e" />
          <FloatingPanel.ResizeTrigger axis="w" />
          <FloatingPanel.ResizeTrigger axis="s" />
          <FloatingPanel.ResizeTrigger axis="ne" />
          <FloatingPanel.ResizeTrigger axis="se" />
          <FloatingPanel.ResizeTrigger axis="sw" />
          <FloatingPanel.ResizeTrigger axis="nw" />
        </FloatingPanel.Content>
      </FloatingPanel.Positioner>
    </Portal>
  {/if}
</FloatingPanel>
