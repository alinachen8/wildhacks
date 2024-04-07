<script lang="ts">
  import { goto } from '$app/navigation';
  // Import any additional dependencies if needed

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

<div class="page-container bg-custom-color flex justify-center items-center h-screen">
  <form on:submit|preventDefault={login} class="grid grid-cols-2 gap-4 bg-gray-100 p-8 rounded-lg shadow-md">
    <div class="col-span-1">
      <div class="form-group">
        <label for="firstname" class="text-sm font-bold">First Name</label>
        <input 
          type="text"
          required
          id="firstname"
          name="firstname"
          placeholder="Enter your first name"
          bind:value={firstname}
          class="w-full py-2 px-3 mt-1 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-200"
        />
      </div>
      <div class="form-group">
        <label for="lastname" class="text-sm font-bold">Last name</label>
        <input 
          type="text"
          required
          id="lastname"
          name="lastname"
          placeholder="Enter your last name"
          bind:value={lastname}
          class="w-full py-2 px-3 mt-1 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-200"
        />
      </div>
      <div class="form-group">
        <label for="dob" class="text-sm font-bold">Your date of birth</label>
        <input 
          type="date"
          required
          id="dob"
          name="dob"
          placeholder="Enter your date of birth"
          bind:value={dob}
          class="w-full py-2 px-3 mt-1 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-200"
        />
      </div>
    </div>
    <div class="col-span-1">
      <div class="form-group">
        <label for="city" class="text-sm font-bold">Your city</label>
        <input 
          type="text"
          required
          id="city"
          name="city"
          placeholder="Enter your city"
          bind:value={city}
          class="w-full py-2 px-3 mt-1 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-200"
        />
      </div>
      <div class="form-group">
        <label for="email" class="text-sm font-bold">Email</label>
        <input 
          type="email"
          required
          id="email"
          name="email"
          placeholder="Enter your email"
          bind:value={email}
          class="w-full py-2 px-3 mt-1 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-200"
        />
      </div>
      <div class="form-group">
        <label for="password" class="text-sm font-bold">Your password</label>
        <input 
          type="password"
          required
          id="password"
          name="password"
          placeholder="Enter your password"
          bind:value={password}
          class="w-full py-2 px-3 mt-1 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-200"
        />
      </div>
    </div>
    <button type="submit" class="col-span-2 bg-green-500 text-white font-bold py-2 px-4 rounded-full hover:bg-green-700">Sign Up</button>
    <p class="col-span-2 text-center text-gray-500 mt-4"> Already have an account? </p>
    <a href="/signin" class="col-span-2 text-center text-blue-600 hover:underline"> Sign in. </a>
  </form>
</div>

<svelte:head>
  <title>Sign Up</title>
</svelte:head>

