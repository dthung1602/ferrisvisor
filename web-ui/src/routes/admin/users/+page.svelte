<script lang="ts">
  import { User } from "lucide-svelte";
  import { api } from "$lib";
  import AdminListingPage from "$lib/components/AdminListingPage.svelte";
</script>

<AdminListingPage
  title="Users"
  entityName="user"
  entityPath="users"
  displayFields={["id", "email", "is_admin"]}
  icon={User}
  api={api.user}
/>
