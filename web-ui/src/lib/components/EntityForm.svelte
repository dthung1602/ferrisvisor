<script lang="ts" generics="T extends Record<string, number|string>, U">
  import type { RelatedSelect } from "$lib/components/types";
  import { normalizedFieldNameToDisplay } from "$lib/string";

  type FormData = Omit<T, "id" | "created_at" | "updated_at">;

  type Props = {
    formData: FormData;
    relatedSelects: Record<string, RelatedSelect<U>>;
    error?: string;
    onSubmit: (formData: FormData) => Promise<void>;
  };

  type SelectOption = {
    display: string;
    id: number;
  };

  let prop: Props = $props();
  let formData = $derived(prop.formData);
  let error = $derived(prop.error);

  function handleSubmit() {
    if (Object.values(formData).some((v) => !v)) {
      error = "Please fill in all fields";
      return;
    }
    error = "";
    prop.onSubmit(formData);
  }

  // map from field -> select options
  let relatedSelectData: Record<string, SelectOption[]> = $state(Object.fromEntries(
    Object.keys(prop.relatedSelects || {}).map(fieldName => [fieldName, []])
  ));


  $effect(() => {
    Object.entries(prop.relatedSelects).forEach(([fieldName, { listApi, displayFunc, idFunc }]) => {
      listApi().then((things) => {
        relatedSelectData[fieldName] = things.map(
          (thing) =>
            ({
              id: idFunc(thing),
              display: displayFunc(thing)
            })
        );
      });
    });
  });
</script>

<form>
  {#each Object.keys(formData) as key (key)}
    <label class="label">
      <span class="label-text">{normalizedFieldNameToDisplay(key)}</span>
      {#if Object.hasOwn(relatedSelectData, key)}
        <select class="select" bind:value={formData[key]}>
          {#each relatedSelectData[key] as opt (opt.id)}
            <option value="{opt.id}">
              {opt.display}
            </option>
          {/each}
        </select>
      {:else}
        <input class="input" bind:value={formData[key]} type={typeof formData[key] === "number" ? "number" : "text"} />
      {/if}
    </label>
  {/each}
  <button onclick={handleSubmit} type="button" class="btn preset-filled-primary-500">Create</button>
  {#if error}
    <p class="mt-1 text-sm text-error-500">{error}</p>
  {/if}
</form>
