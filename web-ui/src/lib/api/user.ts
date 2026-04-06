export type User = {
  id: number;
  email: string;
  is_admin: boolean;
  last_login: string;
  created_at: string;
  updated_at: string;
};

export type NewUser = Omit<User, "id" | "created_at" | "updated_at" | "last_login">;

async function list(): Promise<User[]> {
  const resp = await fetch("/api/user");

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as User[];
}

async function get(userId: number): Promise<User> {
  const resp = await fetch(`/api/user/${userId}`);

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as User;
}

async function create(user: NewUser): Promise<User> {
  const resp = await fetch("/api/user", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(user)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as User;
}

async function update(userId: number, user: NewUser): Promise<User> {
  const resp = await fetch(`/api/user/${userId}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(user)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as User;
}

async function remove(userId: number): Promise<void> {
  const resp = await fetch(`/api/user/${userId}`, {
    method: "DELETE"
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }
}

export default { list, get, create, update, remove };
