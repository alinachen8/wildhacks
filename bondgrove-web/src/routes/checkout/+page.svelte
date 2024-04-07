<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Bond Checkout</title>
  <style>
    /* Add styles for the bond checkout page */
    .container {
      max-width: 600px;
      margin: 0 auto;
      padding: 20px;
    }

    .bond-details {
      margin-bottom: 20px;
    }

    .bond-details h2 {
      margin-bottom: 10px;
    }

    .bond-details p {
      margin: 5px 0;
    }

    .checkout-button {
      background-color: #007bff;
      color: #fff;
      border: none;
      border-radius: 4px;
      padding: 10px 20px;
      cursor: pointer;
    }

    .checkout-button:hover {
      background-color: #0056b3;
    }
  </style>
</head>
<body>
  <div class="container">
    <!-- Bond details section -->
    <div class="bond-details">
      <h2>Bond Name: ABC Corporation Bond</h2>
      <p>Annual Percentage Yield (APY): 3.5%</p>
      <p>Price: $1000</p>
      <p>Term: 5 years</p>
      <p>Description: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut elit tellus, luctus nec ullamcorper mattis, pulvinar dapibus leo.</p>
    </div>

    <!-- Checkout button -->
    <button class="checkout-button">Proceed to Checkout</button>
  </div>
</body>
</html>
