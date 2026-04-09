<script lang="ts">
  import { resolve } from "$app/paths";
</script>

<h3>Admin pages</h3>
<nav>
  <a class="" href={resolve("/admin/groups")}>Groups</a>
  <a href={resolve("/admin/hosts")}>Hosts</a>
  <a href={resolve("/admin/users")}>Users</a>
</nav>
