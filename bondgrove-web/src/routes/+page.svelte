<script>
    import LoginHeader from "../components/LoginHeader.svelte";
    
 

   
   



</script>



<img src="/images/bondgrove_logo_pic.webp" alt="logo">



<div class="index">
  <LoginHeader />
</div>

<div class="slogan">
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

  :global(.slogan h3) {
    font-size: 1.5rem; /* Adjust the size as needed */
    display: flex;
    flex-direction: column;
    align-items: center; /* Center horizontally */
    justify-content: flex-start; /* Align content to the top */
    height: 15vh; /* Make the div take up half the height of the viewport */
    margin-top: 20px;
  }

  :global(.slogan p) {
    font-size: 1rem; /* Adjust the size as needed */
    text-align: center; /* Center the text horizontally */
    padding-top: 0; /* Adjust if you need space above the paragraph */
    margin-top: 0; /* Adjust if you need to move the paragraph up */
    line-height: 1.5;
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

<div style="position: relative; height: 50vh; bottom: 0; width: 100%;">
  <img src="/images/homepage_pic.webp" alt="Homepage pic" style="width: 200px; height: auto; margin: auto; position: absolute; bottom: 50%; left: 0; right: 0;">
</div>


