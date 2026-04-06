<script lang="ts">
  import EntityForm from "$lib/components/EntityForm.svelte";
  import type { User } from "$lib/api/user";
  import { user } from "$lib/api";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { omit } from "lodash";

  type NewUser = Omit<User, "id" | "created_at" | "updated_at" | "last_login">;

  const userId = parseInt(page.params.id || "0");

  let error = $state("");

  async function onSubmit(formData: NewUser) {
    try {
      await user.update(userId, formData);
      await goto(resolve("/admin/users"));
    } catch (e) {
      console.error(e);
      error = e + "";
    }
  }

  let formData: NewUser = $state({
    email: "",
    is_admin: "",
  });

  $effect(() => {
    user.get(userId).then((userData) => {
      formData = omit(userData, ["id", "created_at", "updated_at"]);
    })
  })
</script>

<EntityForm {formData} {error} {onSubmit} relatedSelects={{}} actionBtnText="Update" />
