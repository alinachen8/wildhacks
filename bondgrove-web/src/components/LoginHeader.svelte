<!-- component for header login, includes logo and signin button -->
<script>
  import { goto } from '$app/navigation';

  function navToSignIn() {
    goto('/signin');
  }
</script>

<header class="flex flex-col relative x-20">
  <div class="max-w-[1400px] mx-auto w-full flex items-center justify-between p-4 py-6">
    <!-- set the page route to landing page -->
    <a href="/">
    <h1 class="font-semibold">
      BondGrove
    </h1>
    </a>
    
    <!-- signin buttons -->
    <button on:click={() => goto('/signin')} class="btn text-xs signin-button duration-200 cursor-pointer">
      Sign In
    </button>

    <button on:click={() => goto('/signup')} class="btn text-xs signup-button duration-200 cursor-pointer">
      Sign Up
    </button>
  </div> 

</header>

<style>
  .btn {
    @apply font-bold py-1 px-2 rounded;
    @apply bg-green-400 text-white;
  }
</style>