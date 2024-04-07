<script>
    import LoginHeader from "../components/LoginHeader.svelte";

 

  import imageSrc from 'Users\yaboy\Downloads\bondgrove_logo.webp';
</script>

<img src={imageSrc} alt="Descriptive text for accessibility">


<div class="index">
  <LoginHeader />
  <h3>From Seedling to Skyline: Grow Investments, Grow Communities</h3>
  <p>
    Empowering your investments to support local projects and municipal growth. Where your finances fuel the future.
  </p>
</div>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.green.100);
    font-size: 1.5rem;
  }
 
    :global(.index h1) {
    font-size: 2rem; /* Adjust the size as needed */

  }
  
  :global(.signin-button) {
    /* Tailwind utility class for margin-left auto will push the button to the right */
    font-size: .75rem;
    margin-left: 850px;
  }

  :global(.signup-button) {
    /* Tailwind utility class for margin-left auto will push the button to the right */
    font-size: .75rem;
    margin-right: 25px;
  }


</style>


