<script lang="ts">
  type Prop = {
    svgFile: string;
    altName?: string;
    size?: "sm" | "md" | "lg";
  };
  const { svgFile, altName, size }: Prop = $props();
  const sizeClass = $derived({ sm: "w-4 h-4", md: "w-6 h-6", lg: "w-8 h-8" }[size ?? "sm"]);
</script>

<img src={svgFile} alt={altName ?? svgFile} class={sizeClass} />
