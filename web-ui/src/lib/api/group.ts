export type Group = {
  id: number;
  name: string;
  description: string;
  created_at: string;
  updated_at: string;
}

export type NewGroup = Omit<Group, "id" | "created_at" | "updated_at">;


async function list(): Promise<Group[]> {
  const resp = await fetch("/api/group");

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Group[];
}

async function get(groupId: number): Promise<Group> {
  const resp = await fetch(`/api/group/${groupId}`);

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Group;
}

async function create(group: NewGroup): Promise<Group> {
  const resp = await fetch("/api/group", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(group)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Group;
}

async function update(groupId: number, group: NewGroup): Promise<Group> {
  const resp = await fetch(`/api/group/${groupId}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(group)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Group;
}

async function remove(groupId: number): Promise<void> {
  const resp = await fetch(`/api/group/${groupId}`, {
    method: "DELETE",
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }
}

export default { list, get, create, update, remove };
