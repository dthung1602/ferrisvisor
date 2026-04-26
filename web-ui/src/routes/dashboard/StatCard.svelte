<script lang="ts">
  import type { Icon as LucideIcon } from "@lucide/svelte";

  type ColorVariant = "primary" | "secondary" | "tertiary" | "success" | "info" | "warning" | "error" | "surface";

  const colorVariantClasses: Record<ColorVariant, { text: string; from: string }> = {
    primary: {
      text: "text-primary-300",
      from: "from-primary-500/20"
    },
    secondary: {
      text: "text-secondary-300",
      from: "from-secondary-500/20"
    },
    tertiary: {
      text: "text-tertiary-300",
      from: "from-tertiary-500/20"
    },
    success: {
      text: "text-success-300",
      from: "from-success-500/20"
    },
    info: {
      text: "text-info-300",
      from: "from-info-500/20"
    },
    warning: {
      text: "text-warning-300",
      from: "from-warning-500/20"
    },
    error: {
      text: "text-error-300",
      from: "from-error-500/20"
    },
    surface: {
      text: "text-surface-300",
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

  let colors = $derived(colorVariantClasses[colorVariant]);
</script>

<div
  class="group relative flex min-h-35 flex-col justify-between overflow-hidden rounded-xl border border-surface-500/10 bg-surface-50-950/80 p-5 backdrop-blur-sm transition-all hover:bg-surface-500/10"
>
  <div
    class="absolute inset-0 bg-linear-to-br {colors.from} to-transparent opacity-0 transition-opacity group-hover:opacity-100"
  ></div>

  <div class="mb-2 flex items-start justify-between">
    <span class="text-xs font-bold tracking-widest text-inherit uppercase">{title}</span>
    <StatIcon class="{colors.text} size-6" />
  </div>

  <div class="flex items-baseline gap-2">
    <span class="text-4xl font-bold {colors.text}">{value}</span>
  </div>
</div>
