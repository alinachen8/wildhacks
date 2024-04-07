<script>
  import LoginHeader from "../components/LoginHeader.svelte";
  import { AccordionItem, Accordion } from 'flowbite-svelte';

  import pic from "$lib/images/bg1.png";
</script> 


<!-- <div class="bg-[#fbe9d3]" style="width: 100vw; height: 100vh;"> -->
  <!-- <div class="bg-image-container">
    <img src={pic} alt="logo" class="center-bottom-image"/>
  </div> -->

<div class="bg-[#fbe9d3]" style="width: 100vw; height: 100vh;">
  <div class="index">
    <LoginHeader />
  </div>

  <div class="slogan-container">
    <div class="slogan">
      <h3>From Seedling to Skyline: Grow Investments, Grow Communities</h3>
      <p>
        Empowering your investments to support local projects and municipal growth. Grow your portfolio while nurturing the places you care about. Invest well and make an impact. Welcome to the future of meaningful investing.
      </p>
    </div>
  </div>
  

  <div class="bg-image-container absolute bottom-0 left-0 w-full flex justify-center">
    <img src={pic} alt="logo" class="center-bottom-image"/>
  </div>
</div>

<style lang="postcss">
  :global(html) {
    font-size: 1.5rem;
    background-color: #fbe9d3; /* Updated background color */
  }

  :global(body) {
    @apply bg-green-100;
  }

  :global(.index h1) {
    font-size: 2rem;
  }

  :global(.slogan h3) {
    font-size: 1.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    height: 15vh;
    margin-top: 20px;
  }

  :global(.slogan p) {
    font-size: 1rem;
    text-align: center;
    padding-top: 0;
    margin-top: 0;
    line-height: 1.5;
  }

  :global(.signin-button) {
    font-size: .75rem;
    margin-left: auto;
  }

  :global(.signup-button) {
    font-size: .75rem;
    margin-right: 25px;
  }

  .bg-image-container {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 50vh;
    overflow: hidden;
  }

  .slogan-container {
  width: 80%; /* Adjust the width as needed */
  margin: 0 auto; /* Center the container horizontally */
}

.slogan {
  text-align: center; /* Center the text */
  padding: 20px; /* Add padding to prevent text from touching the edges */
  margin-top: -15px; /* Adjust the top margin to move the text up */
}


</style>
