<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Bond Options</title>
  <style>
    /* Container for the cards */
    .card-container {
      display: flex;
      flex-wrap: wrap;
      justify-content: space-around;
      align-items: flex-start;
      margin-top: 20px;
    }

    /* Individual card styles */
    .card {
      width: calc(25% - 20px);
      background-color: #fff;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      padding: 20px;
      margin: 10px;
    }

    .card h2 {
      font-size: 18px;
      margin-bottom: 10px;
    }

    .card p {
      font-size: 14px;
      color: #666;
    }

    .card button {
      background-color: #007bff;
      color: #fff;
      border: none;
      border-radius: 4px;
      padding: 8px 16px;
      cursor: pointer;
    }

    .card button:hover {
      background-color: #0056b3;
    }
  </style>
</head>
<body>
  <div class="card-container flex border border-2">
    <!-- Example of multiple bond option cards -->
    <div class="card">
      <h2>Bond Option 1</h2>
      <p>Description of Bond Option 1. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>
      <button>Invest Now</button>
    </div>

    <div class="card">
      <h2>Bond Option 2</h2>
      <p>Description of Bond Option 2. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>
      <button>Invest Now</button>
    </div>

    <div class="card">
      <h2>Bond Option 3</h2>
      <p>Description of Bond Option 3. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
      <button>Invest Now</button>
    </div>

    <div class="card">
      <h2>Bond Option 4</h2>
      <p>Description of Bond Option 4. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>
      <button>Invest Now</button>
    </div>

    <!-- Repeat for more cards -->
  </div>
</body>
</html>
