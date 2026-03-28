<script lang="ts">
  import { Server } from "lucide-svelte";
  import { api } from "$lib";
  import AdminListingPage from "$lib/components/AdminListingPage.svelte";
</script>

<AdminListingPage
  title="Hosts"
  entityName="host"
  entityPath="hosts"
  displayFields={["id", "name"]}
  icon={Server}
  api={api.host}
/>
