<script lang="ts">
  import type { Icon as LucideIcon } from "@lucide/svelte";

  type ColorVariant = "primary" | "secondary" | "tertiary" | "success" | "info" | "warning" | "error" | "surface";

  const COLOR_VARIANT_CLASSES: Record<ColorVariant, { text: string; from: string }> = {
    primary: {
      text: "text-primary-700-300",
      from: "from-primary-500/20"
    },
    secondary: {
      text: "text-secondary-700-300",
      from: "from-secondary-500/20"
    },
    tertiary: {
      text: "text-tertiary-700-300",
      from: "from-tertiary-500/20"
    },
    success: {
      text: "text-success-700-300",
      from: "from-success-500/20"
    },
    info: {
      text: "text-info-700-300",
      from: "from-info-500/20"
    },
    warning: {
      text: "text-warning-700-300",
      from: "from-warning-500/20"
    },
    error: {
      text: "text-error-700-300",
      from: "from-error-500/20"
    },
    surface: {
      text: "text-surface-700-300",
      from: "from-surface-500/20"
    }
  };

  type Props = {
    title: string;
    value: string | number;
    colorVariant: ColorVariant;
    icon: typeof LucideIcon;
  };

  let { title, value, colorVariant, icon: StatIcon }: Props = $props();

  let colors = $derived(COLOR_VARIANT_CLASSES[colorVariant]);
</script>

<div
  class="group relative flex min-h-35 flex-col justify-between overflow-hidden rounded-xl border border-surface-500/20 bg-white p-5 backdrop-blur-sm transition-all hover:bg-surface-500/10 dark:bg-surface-800/50"
>
  <div
    class="absolute inset-0 bg-linear-to-bl {colors.from} to-[rgba(255,255,255,0.50)] opacity-0 transition-opacity group-hover:opacity-100 dark:to-transparent"
  ></div>

  <div class="mb-2 flex items-start justify-between">
    <span class="text-xs font-bold tracking-widest text-inherit uppercase">{title}</span>
    <StatIcon class="{colors.text} size-6" />
  </div>

  <div class="flex items-baseline gap-2">
    <span class="text-4xl font-bold {colors.text}">{value}</span>
  </div>
</div>
