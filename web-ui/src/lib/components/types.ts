import type { Host } from "$lib/api/host";
import type { Group } from "$lib/api/group";
import type { User } from "$lib/api/user";
import type { Permission } from "$lib/api/permission";

export type Entity = Host | Group | User | Permission;

export type RelatedSelect<T> = {
  listApi: () => Promise<T[]>;
  idFunc: (thing: T) => number;
  displayFunc: (thing: T) => string;
};
