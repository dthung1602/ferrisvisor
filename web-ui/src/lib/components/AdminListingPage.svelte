<script lang="ts" generics="T extends Entity">
  import type { Entity } from "./types.ts";
  import type { Icon as LucideIcon } from "lucide-svelte";

  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  type F = keyof T;

  type API = {
    list: () => Promise<T[]>;
    remove: (id: number) => Promise<void>;
  };

  type Props = {
    title: string;
    entityName: "host" | "group" | "user";
    entityPath: "hosts" | "groups" | "users";
    displayFields: F[];
    icon: typeof LucideIcon;
    api: API,
  };

  const { title, entityPath, entityName, icon: EntityIcon, displayFields, api }: Props = $props();

  let entities: T[] = $state([]);

  function fetchEntities() {
    api.list().then((fetchedEntities: T[]) => {
      entities = fetchedEntities;
    });
  }

  $effect(fetchEntities);

  function goToNewPage() {
    goto(resolve(`/admin/${entityPath}/new`));
  }

  function goToEditPage(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const entityId = target.dataset.id;
    goto(resolve(`/admin/${entityPath}/edit/${entityId}`));
  }

  function removeEntity(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const entityId = parseInt(target.dataset.id as string);
    if (confirm(`Are you sure you want to remove ${entityName} with id ${entityId} ?`)) {
      api.remove(entityId).then(fetchEntities)
    }
  }
</script>

<div class="flex justify-between">
  <h1>{title}</h1>
  <button onclick={goToNewPage} type="button" class="btn preset-filled-success-500">
    <EntityIcon size="20" />
    Add {entityName}
  </button>
</div>

<div class="table-wrap">
  <table class="table">
    <tbody class="[&>tr]:hover:preset-tonal-primary">
      {#each entities as entity (entity.id)}
        <tr>
          {#each displayFields as field (field)}
            <td>{entity[field]}</td>
          {/each}
          <td class="text-right">
            <button data-id={entity.id} onclick={goToEditPage} type="button" class="btn preset-filled-secondary-500">
              Edit
            </button>
            <button data-id={entity.id} onclick={removeEntity} type="button" class="btn preset-filled-error-500">
              Delete
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
