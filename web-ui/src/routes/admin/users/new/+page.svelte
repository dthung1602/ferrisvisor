<script lang="ts">
  import EntityForm from "$lib/components/EntityForm.svelte";
  import type { User } from "$lib/api/user";
  import { user } from "$lib/api";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  type NewUser = Omit<User, "id" | "created_at" | "updated_at" | "last_login">;

  let error  = $state("");

  async function onSubmit(formData: NewUser) {
    try {
      await user.create(formData);
      await goto(resolve("/admin/users"));
    } catch (e) {
      console.error(e);
      error = e + ""
    }
  }

  const formData = {
    email: "",
    is_admin: "",
  } as NewUser;
</script>

<EntityForm
  formData={formData}
  error={error}
  onSubmit={onSubmit}
  relatedSelects={{}}
/>
