<script>
  import logo from "$lib/images/logo.png";
</script>

<div class="bg-[#fbe9d3] container mx-auto p-4">
  <header class="flex justify-between items-center mb-4">
    <img src={logo} alt="BondGrove Logo" class="logo" />
    <div class="flex items-center">
      <span class="font-semibold mr-2">Ryan Field</span>
      <button class="compare-button">Show a Comparison</button>
    </div>
  </header>

  <main class="grid md:grid-cols-2 gap-4">
    <div class="recent-updates bg-green-100 p-4 rounded">
      <h3 class="text-lg font-semibold mb-3">Recent updates:</h3>
      <div class="update-item mb-2">
        <!-- Image and description of update -->
      </div>
      <div class="update-item">
        <!-- Image and description of update -->
      </div>
    </div>
    
    <div class="information bg-green-100 p-4 rounded">
      <h3 class="text-lg font-semibold mb-3">Information:</h3>
      <ul class="info-list">
        <li class="mb-1">Title: Ryan Field Football Stadium Evanston, Illinois</li>
        <li class="mb-1">Revenue Bond Series: 2024 Series A</li>
        <li class="mb-1">Denomination: $5,000</li>
        <li class="mb-1">Interest Rate: 3.5% per annum</li>
        <li class="mb-1">Interest Payment Dates: Payable semi-annually on June 1st and December 1st</li>
        <li>Maturity Date: December 1st, 2044</li>
      </ul>
    </div>
  </main>

  <div class="actions flex justify-center my-4">
    <button class="buy-button mr-2">Buy Bonds</button>
    <button class="sell-button">Sell Bonds</button>
  </div>
</div>

<footer class="footer bg-green-500 p-4 mt-4 text-center">
  <span>BONDGROVE</span>
</footer>

<style>
  .logo {
    width: 100px; /* Adjust as needed */
  }

  .compare-button, .buy-button, .sell-button {
    background-color: #84cc16; /* Tailwind green-500 */
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 9999px; /* Fully rounded */
    cursor: pointer;
  }

  .compare-button:hover, .buy-button:hover, .sell-button:hover {
    background-color: #6fa914; /* Darker green for hover state */
  }

  .update-item {
    /* Add styles for update items, such as padding and margin */
  }

  .info-list {
    list-style: none;
    padding: 0;
  }

  .info-list li {
    background: #def7ec; /* Tailwind green-100 */
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    border-radius: 9999px; /* Fully rounded */
  }

  .footer {
    background-color: #047857; /* Tailwind green-700 */
    color: white;
  }
</style>
