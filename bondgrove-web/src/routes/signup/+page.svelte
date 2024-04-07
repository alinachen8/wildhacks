<script lang="ts">
  import { goto } from '$app/navigation';
  import { Alert } from 'flowbite-svelte';

  let firstname: string; 
  let lastname: string;
  let dob: string;
  let city: string; 
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
        <label
          for="firstname"
          >First Name</label>
        <input 
          type="text"
          required
          id="firstname"
          name="firstname"
          placeholder="Enter your first name"
          bind:value={firstname}
        />
      </div>
      <div>
        <label
          for="lastname"
          >Last name</label>
        <input 
          type="text"
          required
          id="lastname"
          name="lastname"
          placeholder="Enter your last name"
          bind:value={lastname}
        />
      </div>
      <div>
        <label
          for="dob"
          >Your date of birth</label>
        <input 
          type="date"
          required
          id="dob"
          name="dob"
          placeholder="Enter your date of birth"
          bind:value={dob}
        />
      </div>
      <div>
        <label
          for="city"
          >Your city</label>
        <input 
          type="text"
          required
          id="city"
          name="city"
          placeholder="Enter your city"
          bind:value={city}
        />
      </div>
      <div>
        <label for="email">
          Email
        </label>
        <input 
          type="email"
          required
          id="email"
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
          required
          id="password"
          name="password"
          placeholder="Enter your password"
          bind:value={password}
        />
      </div>
      <button
        type="submit"
      >Log in</button>
      <p> Already have an account? </p>
      <a href="/signin"> Sign in. </a>
    </form>
  </div>
</div>

<svelte:head>
  <title>Sign Up</title>
</svelte:head>