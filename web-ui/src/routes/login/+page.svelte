<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { api, cookies } from "$lib";
  import logo from "$lib/assets/Original_Ferris.svg";
  import { Mail, LockKeyhole, ShieldCheck, CircleArrowRight } from "lucide-svelte";
  import GoogleIcon from "$lib/assets/icons/google.svg";
  import GithubIcon from "$lib/assets/icons/github.svg";
  import KeyCloak from "$lib/assets/icons/keycloak.svg";
  import { VERSION } from "$lib/constants";
  import CustomIcon from "$lib/components/CustomIcon.svelte";

  let email: string = $state("");
  let password: string = $state("");
  let error: string = $state("");

  async function login(e: Event) {
    e.preventDefault();
    try {
      const loginResult = await api.auth.login(email, password);
      cookies.setSessionToken(loginResult.token, loginResult.expires_at);
      await goto(resolve("/"));
    } catch (e: unknown) {
      error = e instanceof Error ? (e.message ?? "Unknown error") : (e ?? "Unknown error").toString();
    }
  }
</script>

<div class="relative min-h-[calc(100vh-var(--header-height))] flex items-center justify-center p-6 overflow-hidden">
  <!-- Visual Atmosphere Elements -->
  <div class="absolute inset-0 pointer-events-none -z-10">
    <div class="absolute top-[10%] left-[5%] w-96 h-96 bg-primary-500/10 rounded-full blur-[120px]"></div>
    <div class="absolute bottom-[10%] right-[5%] w-120 h-120 bg-secondary-500/10 rounded-full blur-[150px]"></div>
    <div class="absolute inset-0 opacity-[0.03]" style="background-image: radial-gradient(var(--color-primary-500) 1px, transparent 1px); background-size: 40px 40px;"></div>
  </div>

  <main class="w-full max-w-md">
    <!-- Brand Identity Section -->
    <div class="flex flex-col items-center mb-10 text-center">
      <div class="w-24 h-24 mb-6 relative group">
        <div class="absolute inset-0 bg-linear-to-br from-primary-500 to-tertiary-500 rounded-full blur-2xl opacity-20 group-hover:opacity-40 transition-opacity"></div>
        <img src={logo} alt="Ferrisvisor Logo" class="relative w-full h-full object-contain" />
      </div>
      <h1 class="text-3xl font-black tracking-tighter uppercase mb-2">
       System &nbsp; Login
      </h1>
      <p class="text-[10px] tracking-widest text-secondary-500 uppercase font-medium">
        Ferrisvisor v{VERSION}
      </p>
    </div>

    <!-- Login Container -->
    <div class="card p-8 rounded-xl shadow-2xl border border-surface-500/10 bg-surface-50-950/40 backdrop-blur-xl">
      <form class="space-y-6" onsubmit={login}>
        <!-- Credentials Group -->
        <div class="space-y-4">
          <div>
            <label class="block text-[10px] uppercase tracking-widest font-bold mb-2 ml-1 opacity-70" for="email">
              Email
            </label>
            <div class="relative">
              <Mail class="absolute left-4 top-1/2 -translate-y-1/2 opacity-40 size-4" />
              <input
                class="input pl-11 py-3.5 bg-surface-500/10 border-none rounded-xl focus:ring-2 focus:ring-primary-500/40"
                id="email"
                type="email"
                bind:value={email}
                required
              />
            </div>
          </div>
          <div>
            <div class="flex justify-between items-center mb-2 ml-1">
              <label class="block text-[10px] uppercase tracking-widest font-bold opacity-70" for="password">
                Password
              </label>
              <a href="#" class="text-[10px] uppercase tracking-widest text-primary-500 font-bold hover:brightness-110 transition-all">
                Forget password?
              </a>
            </div>
            <div class="relative">
              <LockKeyhole class="absolute left-4 top-1/2 -translate-y-1/2 opacity-40 size-4" />
              <input
                class="input pl-11 py-3.5 bg-surface-500/10 border-none rounded-xl focus:ring-2 focus:ring-primary-500/40"
                id="password"
                type="password"
                bind:value={password}
                required
              />
            </div>
          </div>
        </div>

        {#if error}
          <div class="p-3 bg-error-500/10 border border-error-500/20 rounded-lg text-error-500 text-xs text-center">
            {error}
          </div>
        {/if}

        <!-- Primary Action -->
        <button
          class="btn w-full bg-linear-to-br from-primary-500 to-tertiary-500 text-on-primary font-bold py-4 rounded-full uppercase tracking-widest text-sm shadow-lg shadow-primary-500/20 hover:scale-[1.02] active:scale-[0.98] transition-all flex items-center justify-center gap-2"
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
        <div class="relative flex justify-center text-[10px] uppercase tracking-[0.2em]">
          <span class="bg-surface-100-900 px-4 py-1 rounded-full opacity-50 backdrop-blur-sm">Other methods</span>
        </div>
      </div>

      <!-- Social Logins -->
      <div class="grid grid-cols-3 gap-4">
        <button class="flex flex-col items-center justify-center gap-2 bg-surface-500/5 hover:bg-surface-500/10 py-4 rounded-xl transition-all border border-transparent hover:border-surface-500/20">
          <CustomIcon svgFile={GoogleIcon} size="md"/>
          <span class="text-[9px] font-bold opacity-50 uppercase tracking-tighter">Google</span>
        </button>
        <button class="flex flex-col items-center justify-center gap-2 bg-surface-500/5 hover:bg-surface-500/10 py-4 rounded-xl transition-all border border-transparent hover:border-surface-500/20">
          <CustomIcon svgFile={GithubIcon} size="md"/>
          <span class="text-[9px] font-bold opacity-50 uppercase tracking-tighter">GitHub</span>
        </button>
        <button class="flex flex-col items-center justify-center gap-2 bg-surface-500/5 hover:bg-surface-500/10 py-4 rounded-xl transition-all border border-transparent hover:border-surface-500/20">
          <CustomIcon svgFile={KeyCloak} size="md"/>
          <span class="text-[9px] font-bold opacity-50 uppercase tracking-tighter">Keycloak</span>
        </button>
      </div>
    </div>

    <!-- Footer / Redirect -->
    <div class="mt-8 text-center">
      <p class="opacity-50 text-xs flex items-center justify-center gap-2">
        New to Ferrisvisor?
        <a href="#" class="text-primary-500 font-bold hover:underline transition-all uppercase tracking-widest text-[10px]">Create account</a>
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
