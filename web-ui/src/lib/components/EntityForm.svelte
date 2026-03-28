<script lang="ts" generics="T">
  type FormData = Omit<T, "id" | "created_at" | "updated_at">

  type Props = {
    formData: FormData;
    error?: string;
    onSubmit: (formData: FormData) => Promise<void>;
  }

  let prop: Props = $props();
  let formData = $derived(prop.formData);
  let error = $derived(prop.error);

  function handleSubmit() {
    if (Object.values(formData).some((v) => !v)) {
      error = "Please fill in all fields";
      return;
    }
    error = "";
    prop.onSubmit(formData)
  }
</script>

<form>
  {#each Object.keys(formData) as key (key)}
    <label class="label">
      <span class="label-text">{key}</span>
      <input class="input" bind:value={formData[key]} type="{typeof (formData[key]) === 'number' ? 'number' : 'text'}" />
    </label>
  {/each}
  <button onclick={handleSubmit} type="button" class="btn preset-filled-primary-500">Create</button>
  {#if error}
    <p class="mt-1 text-sm text-error-500">{error}</p>
  {/if}
</form>
