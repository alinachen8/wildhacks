<script lang="ts">
  import { goto } from '$app/navigation';
  import { Alert } from 'flowbite-svelte';

  let email: string;
  let password: string;
  let error: string;

  const login = async () => {
    console.log('email: ', email, 'password: ', password);
    goto('/');
  }

</script> 

<div>
  <div>
    <form on:submit|preventDefault={login}>
      <div>
        <label for="email">
          Email
        </label>
        <input 
          type="email"
          id="email"
          required
          name="email"
          placeholder="Enter your email"
          bind:value={email}
        />
      </div>
      <div>
        <label
          for="password"
          >Your password</label>
        <input 
          type="password"
          id="password"
          required
          name="password"
          placeholder="Enter your password"
          bind:value={password}
        />
      </div>
      <button
        type="submit"
      >Log in</button>
      <p> Don't have an account? </p>
      <a href="/signup"> Sign up. </a>
    </form>
  </div>
</div>

<svelte:head>
  <title>Sign In</title>
</svelte:head>