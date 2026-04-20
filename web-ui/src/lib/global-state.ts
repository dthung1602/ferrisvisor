import { createContext } from "svelte";

import type { CurrentUser } from "$lib/api/auth";

export interface GlobalState {
  currentUser: CurrentUser | null;
  isDarkMode: boolean;
}

// createContext returns a [get, set] pair
export const [getGlobalContext, setGlobalContext] = createContext<GlobalState>();
