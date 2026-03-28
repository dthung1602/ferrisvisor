<script lang="ts">
  import { Group } from "lucide-svelte";
  import { api } from "$lib";
  import AdminListingPage from "$lib/components/AdminListingPage.svelte";
</script>

<AdminListingPage
  title="Groups"
  entityName="group"
  entityPath="groups"
  displayFields={["id", "name", "description"]}
  icon={Group}
  api={api.group}
/>
