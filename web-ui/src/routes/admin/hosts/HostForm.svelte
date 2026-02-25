<script lang="ts">
  import type { NewHost } from "$lib/api/host";

  interface Props extends Partial<NewHost> {
    error?: string;
    onSubmit: (host: NewHost) => void;
  }

  let {
    name: defaultName,
    port: defaultPort,
    username: defaultUsername,
    password: defaultPassword,
    error: defaultError,
    onSubmit
  }: Props = $props();

  let name = $state(defaultName);
  let port = $state(defaultPort);
  let username = $state(defaultUsername);
  let password = $state(defaultPassword);
  let error = $state(defaultError);

  $effect(() => {
    name = defaultName;
    port = defaultPort;
    username = defaultUsername;
    password = defaultPassword;
  });

  function handleSubmit() {
    if (!name || !port || !username || !password) {
      error = "Please fill in all fields";
      return;
    }
    onSubmit({ name, port, username, password })
  }
</script>

<form>
  <label class="label">
    <span class="label-text">Name</span>
    <input class="input" bind:value={name} />
  </label>
  <label class="label">
    <span class="label-text">Port</span>
    <input class="input" type="number" min="1" max="65535" bind:value={port} />
  </label>
  <label class="label">
    <span class="label-text">Login username</span>
    <input class="input" bind:value={username} />
  </label>
  <label class="label">
    <span class="label-text">Login password</span>
    <input class="input" type="password" bind:value={password} />
  </label>
  <button onclick={handleSubmit} type="button" class="btn preset-filled-primary-500">Create</button>
  {#if error}
    <p class="mt-1 text-sm text-error-500">{error}</p>
  {/if}
</form>
