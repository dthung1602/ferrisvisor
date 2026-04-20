<script lang="ts">
  import { CircleArrowRight, LockKeyhole, Mail } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { api, cookies } from "$lib";

  import { page} from "$app/state";
  import GithubIcon from "$lib/assets/icons/github.svg";
  import GoogleIcon from "$lib/assets/icons/google.svg";
  import KeyCloak from "$lib/assets/icons/keycloak.svg";
  import logo from "$lib/assets/Original_Ferris.svg";
  import CustomIcon from "$lib/components/CustomIcon.svelte";
  import { VERSION } from "$lib/constants";
  import { getGlobalContext } from "$lib/global-state";

  let email: string = $state("");
  let password: string = $state("");
  let error: string = $state("");

  let globalContext = getGlobalContext();

  async function login(e: Event) {
    e.preventDefault();
    try {
      const loginUser = await api.auth.login(email, password);
      cookies.setSessionToken(loginUser.session.token, loginUser.session.expires_at);
      globalContext.currentUser = loginUser;

      const params = new URLSearchParams(window.location.search);
      let redirect = params.get("redirect") ?? "/";
      // eslint-disable-next-line svelte/no-navigation-without-resolve
      await goto(redirect);
    } catch (e: unknown) {
      error = e instanceof Error ? (e.message ?? "Unknown error") : (e ?? "Unknown error").toString();
    }
  }
</script>

<div class="relative flex min-h-[calc(100vh-var(--header-height))] items-center justify-center overflow-hidden p-6">
  <!-- Visual Atmosphere Elements -->
  <div class="pointer-events-none absolute inset-0 -z-10">
    <div class="absolute top-[10%] left-[5%] h-96 w-96 rounded-full bg-primary-500/10 blur-[120px]"></div>
    <div class="absolute right-[5%] bottom-[10%] h-120 w-120 rounded-full bg-secondary-500/10 blur-[150px]"></div>
    <div
      class="absolute inset-0 opacity-[0.03]"
      style="background-image: radial-gradient(var(--color-primary-500) 1px, transparent 1px); background-size: 40px 40px;"
    ></div>
  </div>

  <main class="w-full max-w-md">
    <!-- Brand Identity Section -->
    <div class="mb-10 flex flex-col items-center text-center">
      <div class="group relative mb-6 h-24 w-24">
        <div
          class="absolute inset-0 rounded-full bg-linear-to-br from-primary-500 to-tertiary-500 opacity-20 blur-2xl transition-opacity group-hover:opacity-40"
        ></div>
        <img src={logo} alt="Ferrisvisor Logo" class="relative h-full w-full object-contain" />
      </div>
      <h1 class="mb-2 text-3xl font-black tracking-tighter uppercase">System &nbsp; Login</h1>
      <p class="text-[10px] font-medium tracking-widest text-secondary-500 uppercase">
        Ferrisvisor v{VERSION}
      </p>
    </div>

    <!-- Login Container -->
    <div class="card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-8 shadow-2xl backdrop-blur-xl">
      <form class="space-y-6" onsubmit={login}>
        <!-- Credentials Group -->
        <div class="space-y-4">
          <div>
            <label class="mb-2 ml-1 block text-[10px] font-bold tracking-widest uppercase opacity-70" for="email">
              Email
            </label>
            <div class="relative">
              <Mail class="absolute top-1/2 left-4 size-4 -translate-y-1/2 opacity-40" />
              <input
                class="input rounded-xl border-none bg-surface-500/10 py-3.5 pl-11 focus:ring-2 focus:ring-primary-500/40"
                id="email"
                type="email"
                bind:value={email}
                required
              />
            </div>
          </div>
          <div>
            <div class="mb-2 ml-1 flex items-center justify-between">
              <label class="block text-[10px] font-bold tracking-widest uppercase opacity-70" for="password">
                Password
              </label>
              <a
                href="#"
                class="text-[10px] font-bold tracking-widest text-primary-500 uppercase transition-all hover:brightness-110"
              >
                Forget password?
              </a>
            </div>
            <div class="relative">
              <LockKeyhole class="absolute top-1/2 left-4 size-4 -translate-y-1/2 opacity-40" />
              <input
                class="input rounded-xl border-none bg-surface-500/10 py-3.5 pl-11 focus:ring-2 focus:ring-primary-500/40"
                id="password"
                type="password"
                bind:value={password}
                required
              />
            </div>
          </div>
        </div>

        {#if error}
          <div class="rounded-lg border border-error-500/20 bg-error-500/10 p-3 text-center text-xs text-error-500">
            {error}
          </div>
        {/if}

        <!-- Primary Action -->
        <button
          class="text-on-primary btn flex w-full items-center justify-center gap-2 rounded-full bg-linear-to-br from-primary-500 to-tertiary-500 py-4 text-sm font-bold tracking-widest uppercase shadow-lg shadow-primary-500/20 transition-all hover:scale-[1.02] active:scale-[0.98]"
          type="submit"
        >
          Login
          <CircleArrowRight class="size-4" />
        </button>
      </form>

      <!-- Divider -->
      <div class="relative my-10">
        <div class="absolute inset-0 flex items-center">
          <div class="w-full border-t border-surface-500/10"></div>
        </div>
        <div class="relative flex justify-center text-[10px] tracking-[0.2em] uppercase">
          <span class="rounded-full bg-surface-100-900 px-4 py-1 opacity-50 backdrop-blur-sm">Other methods</span>
        </div>
      </div>

      <!-- Social Logins -->
      <div class="grid grid-cols-3 gap-4">
        <button
          class="flex flex-col items-center justify-center gap-2 rounded-xl border border-transparent bg-surface-500/5 py-4 transition-all hover:border-surface-500/20 hover:bg-surface-500/10"
        >
          <CustomIcon svgFile={GoogleIcon} size="md" />
          <span class="text-[9px] font-bold tracking-tighter uppercase opacity-50">Google</span>
        </button>
        <button
          class="flex flex-col items-center justify-center gap-2 rounded-xl border border-transparent bg-surface-500/5 py-4 transition-all hover:border-surface-500/20 hover:bg-surface-500/10"
        >
          <CustomIcon svgFile={GithubIcon} size="md" />
          <span class="text-[9px] font-bold tracking-tighter uppercase opacity-50">GitHub</span>
        </button>
        <button
          class="flex flex-col items-center justify-center gap-2 rounded-xl border border-transparent bg-surface-500/5 py-4 transition-all hover:border-surface-500/20 hover:bg-surface-500/10"
        >
          <CustomIcon svgFile={KeyCloak} size="md" />
          <span class="text-[9px] font-bold tracking-tighter uppercase opacity-50">Keycloak</span>
        </button>
      </div>
    </div>

    <!-- Footer / Redirect -->
    <div class="mt-8 text-center">
      <p class="flex items-center justify-center gap-2 text-xs opacity-50">
        New to Ferrisvisor?
        <a
          href="#"
          class="text-[10px] font-bold tracking-widest text-primary-500 uppercase transition-all hover:underline"
          >Create account</a
        >
      </p>
    </div>
  </main>
</div>

<style>
  /* Custom glass effect if needed */
  :global(.card) {
    transition: all 0.3s ease-in-out;
  }
</style>
