<script>
  // Function to handle clicking on the "Show a Comparison" button
  function showComparison() {
    // Logic to show a comparison
  }

  // Function to handle clicking on the "Buy Bonds" button
  function buyBonds() {
    // Logic to buy bonds
  }

  // Function to handle clicking on the "Sell Bonds" button
  function sellBonds() {
    // Logic to sell bonds
  }

  // Function to handle clicking on any of the update buttons
  function handleUpdateClick(updateInfo) {
    // Logic to handle update, possibly navigate to a detail page
  }
</script>

<style>
  /* Define your color scheme and layout here */
  .button {
    background-color: #6fa914; /* Replace with actual color code from the mockup */
    border: none;
    border-radius: 5px; /* Adjust as needed for oval shape */
    padding: 2px 2px;
    cursor: pointer;
  }
  .information {
    background-color: #6fa914; /* Replace with actual color code from the mockup */
    border: none;
    border-radius: 50px; /* Adjust as needed for oval shape */
    padding: 20px 20px;
    cursor: pointer;
    width: 40%;
    margin-left: auto;
    margin-right: 30px;
    margin-top: 100px;
    margin-bottom: 10px;

  }

  .actions {
    display: flex;
    justify-content: center; /* This will center the buttons horizontally */
    align-items: center;
    margin-top: 200px;
    
  }

  /* Additional styling... */
</style>


<!-- The main container -->
<div class="main-container">
  <!-- The header section -->
  <div class="header">
    <img class="logo" src="/path-to-logo.png" alt="Logo">
    <h1>RYAN FIELD</h1>
    <button class="button" on:click={showComparison}>SHOW A COMPARISON</button>
  </div>
  
  <!-- The updates section -->
  <div class="updates">
    <h2>Recent updates:</h2>
    <!-- Example update; in practice, you'd map over an array of updates -->
    <div class="update">
      <img src="/path-to-update-image.jpg" alt="Update Image">
      <button class="button" on:click={() => handleUpdateClick('Update Info')}>Update Info</button>
    </div>
    <!-- Repeat for other updates -->
  </div>


  <!-- The information section -->
  <div class="information">
    <div class="info-item">Revenue Bond Series: 2024 Series AA</div>
    <div class="info-item">Minimum Bond Denomination: $5000</div>
    <div class="info-item">Interest Rate: 3.5% Per Annum</div>
    <div class="info-item">Interest Payout Dates: June 1 and December 1</div>
    <div class="info-item">Maturity Date, December 1st, 2044</div>
    <div class="info-item">Credit Rating: AA via S&P, Moody's</div>

    <!-- ...other information items... -->
    <!-- Interest payment dates button -->
    <button class="button">Interest Payment Dates</button>
    <!-- ...other buttons for each piece of information, as needed... -->
  </div>


  <!-- Action buttons for buying and selling bonds -->
  <div class="actions">
    <button class="button" on:click={buyBonds}>BUY BONDS</button>
    <button class="button" on:click={sellBonds}>SELL BONDS</button>
  </div>
</div>
