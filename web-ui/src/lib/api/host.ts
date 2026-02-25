export type Host = {
  id: number;
  name: string;
  port: number;
  username: string;
  password: string;
  created_at: string;
  updated_at: string;
}

export type NewHost = Omit<Host, "id" | "created_at" | "updated_at">;

export type UpdateHost = Omit<Host, "created_at" | "updated_at">;


async function list(): Promise<Host[]> {
  const resp = await fetch("/api/host");

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Host[];
}

async function get(id: number): Promise<Host> {
  const resp = await fetch(`/api/host/${id}`);

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Host;
}

async function create(host: NewHost): Promise<Host> {
  const resp = await fetch("/api/host", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(host)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Host;
}

async function update(host: UpdateHost): Promise<Host> {
  const resp = await fetch("/api/host", {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(host)
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Host;
}

export default { list, get, create, update };
