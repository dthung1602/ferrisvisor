export type LoginResult = {
  email: string;
  is_admin: boolean;
  token: string;
  expires_at: string;
};

async function login(email: string, password: string): Promise<LoginResult> {
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

  return (await resp.json()) as LoginResult;
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

export default { login, logout };
