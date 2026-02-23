<script lang="ts">
    import {goto} from '$app/navigation';
    import {resolve} from "$app/paths";
    import {api, cookies} from "$lib";

    let email: string = $state("");
    let password: string = $state("");
    let error: string = $state("");

    async function login() {
        try {
            console.log("login: ", email, password);
            const loginResult = await api.login(email, password);
            console.log("loginResult: ", loginResult);
            cookies.setSessionToken(loginResult.token, loginResult.expires_at);
            await goto(resolve("/"));
        } catch (e: unknown) {
            console.error("login failed: ", e);
            error = e instanceof Error ? (e.message ?? "Unknown error") : (e ?? "Unknown error").toString();
        }
    }
</script>

<div class="py-32 flex items-center justify-center">
    <div class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 card-hover
                divide-surface-200-800 block divide-y w-128">
        <header class="space-y-4 p-4 font-bold">
            LOGIN
        </header>
        <form class="mx-auto w-full max-w-md space-y-4 py-8">
            <label class="label">
                <span class="label-text">Email</span>
                <input class="input" type="email" bind:value={email}/>
            </label>
            <label class="label">
                <span class="label-text">Password</span>
                <input class="input" type="password" bind:value={password}/>
            </label>
            {#if error}
                <p class="text-error-500 text-sm mt-1">{error}</p>
            {/if}
            <div>
                <button class="btn preset-filled-primary-500" onclick={login}>Login</button>
                <!-- TODO -->
                <a href="#" class="link float-right">Forgot Password?</a>
            </div>
        </form>
    </div>
</div>