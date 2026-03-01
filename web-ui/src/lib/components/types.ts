import type { Host } from "$lib/api/host";
import type { Group } from "$lib/api/group";

export type Entity = Host | Group;

export type AllowedFields<Obj, Type = string> = {
  [K in keyof Obj]: Obj[K] extends Type ? K : never;
}