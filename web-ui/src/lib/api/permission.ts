export type Permission = {
  id: number;
  user_id: number;
  host_id: number | null;
  host_name: string | null;
  group_id: number;
  group_name: string;
  service_name: string;
  can_view: boolean;
  can_act: boolean;
};

export type NewPermission = Omit<Permission, "id" | "user_id" | "host_name" | "group_id" | "group_name">;

async function list(userId: number): Promise<Permission[]> {
  const resp = await fetch(`/api/user/${userId}/permission`);

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Permission[];
}

async function get(userId: number, permissionId: number): Promise<Permission> {
  const resp = await fetch(`/api/user/${userId}/permission/${permissionId}`);

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Permission;
}

async function create(userId: number, permission: NewPermission): Promise<Permission> {
  const resp = await fetch(`/api/user/${userId}/permission`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(permission)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Permission;
}

async function update(userId: number, permissionId: number, permission: NewPermission): Promise<Permission> {
  const resp = await fetch(`/api/user/${userId}/permission/${permissionId}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(permission)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Permission;
}

async function remove(userId: number, permissionId: number): Promise<void> {
  const resp = await fetch(`/api/user/${userId}/permission/${permissionId}`, {
    method: "DELETE"
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }
}

export default { list, get, create, update, remove };
