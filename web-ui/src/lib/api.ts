type LoginResult = {
  email: string;
  is_admin: boolean;
  token: string;
  expires_at: string;
};

async function login(email: string, password: string): Promise<LoginResult> {
  const resp = await fetch(`/api/login`, {
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

export { login };
export type { LoginResult };
