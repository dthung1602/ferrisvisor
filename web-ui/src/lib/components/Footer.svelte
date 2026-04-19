<script lang="ts">
  import { resolve } from "$app/paths";

  import { GITHUB_REPO_URL, VERSION } from "$lib/constants";
</script>

<footer
  class="flex items-center justify-between border-t border-surface-500/10 bg-surface-50-950 px-8 py-2 text-[9px] font-bold tracking-widest text-surface-500 uppercase"
>
  <div class="flex items-center gap-6">
    <span class="text-primary-500">version {VERSION}</span>
  </div>
  <div class="flex items-center gap-6">
    <a href={resolve("/about")}>About</a>
    <a href={GITHUB_REPO_URL} rel="external" target="_blank">Github</a>
  </div>
</footer>
