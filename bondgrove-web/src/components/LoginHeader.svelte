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
    <button on:click={() => goto('/signin')} class="duration-200 hover:text-green-400 cursor-pointer">
      Sign In
    </button>

    <button on:click={() => goto('/signup')} class="duration-200 hover:text-green-400 cursor-pointer">
      Sign Up
    </button>
  </div> 

</header>