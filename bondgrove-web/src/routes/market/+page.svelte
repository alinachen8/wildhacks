<script> 
  import BondCard from "../../components/BondCard.svelte";
  import bridge from "$lib/images/bridge.png";
  import ryanField from "$lib/images/newryanfield.jpeg";
  import greenCity from "$lib/images/greenery.png";
  import school from "$lib/images/school.png";
  import { onMount } from 'svelte';

  let data = {};
  async function fetchData() {
    const response = await fetch('http://localhost:3000');
    if (response.ok) {
      data = await response.json();
      console.log('data: ', data);
    } else {
      console.error('Failed to fetch data');
    }
  }

  onMount(() => {
    fetchData();
  });

</script>

<div class="bg-[#fbe9d3]" style="width: 100vw; height: 100vh;">
  <form class="max-w-md mx-auto p-5">   
    <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
    <div class="relative">
        <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
            <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
            </svg>
        </div>
        <input type="search" id="default-search" class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Search For Bonds..." required />
        <button type="submit" class="text-white absolute end-2.5 bottom-2.5 bg-green-500 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Search</button>
    </div>
  </form>

  <h3 class="mx-5 font-semibold text-xl">Available Bonds: </h3>

  <div class="marketplace p-4">
    <BondCard imgUrl={ryanField} description="Ryan Field Rebuild"/>
    <BondCard imgUrl={bridge} description="New Community Bridge"/>
    <BondCard imgUrl={greenCity} description="Urban Greening"/>
    <BondCard imgUrl={school} description="Build more Schools"/>
  </div>
</div>

<style>
  .marketplace {
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
  }
</style>