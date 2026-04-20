import { redirect } from "@sveltejs/kit";

import { api } from "$lib";

export const prerender = false;

export const ssr = false;

export const trailingSlash = "always";

export async function load({ url }) {
  let currentUser = null;
  try {
    currentUser = await api.auth.me();
  } catch (e) {
    console.error("Failed to fetch current user", e);
  }

  if (!currentUser && url.pathname !== "/login") {
    throw redirect(302, `/login?redirect=${url.pathname}`);
  }

  return { currentUser };
}
