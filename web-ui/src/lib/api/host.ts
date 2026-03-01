export type Host = {
  id: number;
  groupId: number;
  name: string;
  hostname: string;
  port: number;
  username: string;
  password: string;
  created_at: string;
  updated_at: string;
}

export type NewHost = Omit<Host, "id" | "created_at" | "updated_at">;


async function list(): Promise<Host[]> {
  const resp = await fetch("/api/host");

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as Host[];
}

async function get(hostId: number): Promise<Host> {
  const resp = await fetch(`/api/host/${hostId}`);

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

async function update(hostId: number, host: NewHost): Promise<Host> {
  const resp = await fetch(`/api/host/${hostId}`, {
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
