import type { Permission } from "$lib/api/permission";

export type Session = {
  id: number;
  user_id: number;
  token: string;
  expires_at: string;
};

export type CurrentUser = {
  id: number;
  email: string;
  created_at: string;
  updated_at: string;
  last_login: string;
  is_admin: boolean;
  session: Session;
  permissions: Permission[];
};

async function login(email: string, password: string): Promise<CurrentUser> {
  const resp = await fetch("/api/login", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password })
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as CurrentUser;
}

async function logout() {
  const resp = await fetch("/api/logout", { method: "POST" });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return;
}

async function me(): Promise<CurrentUser | null> {
  const resp = await fetch("/api/me");

  if (resp.status === 401 || resp.status === 403  ) {
    return null
  }

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as CurrentUser;
}

export default { login, logout, me };
