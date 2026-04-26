import { api } from "$lib";

import type { Group } from "$lib/api/group";

export async function load() {
  const groups: Group[] = await api.group.list();
  return { groups };
}
