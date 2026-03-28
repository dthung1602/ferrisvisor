import type { Host } from "$lib/api/host";
import type { Group } from "$lib/api/group";

export type Entity = Host | Group;

export type RelatedSelect<T> = {
  listApi: () => Promise<T[]>;
  idFunc: (thing: T) => number;
  displayFunc: (thing: T) => string;
};
