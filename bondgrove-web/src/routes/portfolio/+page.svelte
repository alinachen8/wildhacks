<!DOCTYPE html>
<html lang="en">
<head>
  <!-- Meta tags -->
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  
  <!-- Title -->
  <title>Portfolio</title>
  
  <!-- Load charting library -->
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>

<body>
  <!-- Top right button -->
  <button id="topRightButton" style="
    background-color: #4CAF50; /* Green */
    border: none;
    color: black;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    position: fixed; /* Fixed position */
    top: 250px; /* Distance from the top */
    right: 100px; /* Distance from the right */
    cursor: pointer;
    width: 600px; /* Set the width */
    height: 50px; /* Set the height */
    border-radius: 10px;">Number of Bonds: $$$$$</button>

  <!-- Bottom right button -->
  <button id="BottomRightButton" style="
    background-color: #4CAF50; /* Green */
    border: none;
    color: black;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    position: fixed; /* Fixed position */
    top: 350px; /* Distance from the top */
    right: 100px; /* Distance from the right */
    cursor: pointer;
    width: 600px; /* Set the width */
    height: 50px; /* Set the height */
    border-radius: 10px;">Money Invested: $$$$$</button>

  <!-- Portfolio Overview heading -->
  <h1 style="margin-top: 100px; text-align: center; font-size: 30px;">Portfolio Overview</h1>

  <!-- Pie chart showing diversity -->
  <canvas id="diversityChart" width="400" height="400"></canvas>

  <!-- JavaScript to render charts -->
  <script>
    // Data for the pie chart (example)
    const diversityData = {
      labels: ['Entertainment', 'Roads', 'Housing', 'Hospitals'],
      datasets: [{
        label: 'Diversity',
        data: [30, 40, 20, 10], // Example percentages
        backgroundColor: ['#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0']
      }]
    };

    // Data for the bar chart (example)
    const investmentData = {
      labels: ['Number of Bonds', 'Total Invested'],
      datasets: [{
        label: 'Investment Overview',
        data: [8, 100000], // Example values
        backgroundColor: ['#36A2EB', '#FFCE56']
      }]
    };

    // Configuration options for charts
    const chartOptions = {
      responsive: false, // Adjust based on your needs
      maintainAspectRatio: false, // Adjust based on your needs
      scales: {
        yAxes: [{
          ticks: {
            beginAtZero: true
          }
        }]
      }
    };

    // Render pie chart
    const diversityChart = document.getElementById('diversityChart').getContext('2d');
    new Chart(diversityChart, {
      type: 'pie',
      data: diversityData,
      options: chartOptions
    });

    // Render bar chart
    const investmentChart = document.getElementById('investmentChart').getContext('2d');
    new Chart(investmentChart, {
      type: 'bar',
      data: investmentData,
      options: chartOptions
    });
  </script>

  <!-- Style -->
  <style>
    /* Set the font weight to bold */
    h1 {
      font-weight: bold;
    }

    /* Set the background color for the body */
    body {
      background-color: #fbe9df; /* Your desired background color */
      overflow: hidden; /* Hide scrollbars */
    }

    /* Set the text color and margin for the h2 element */
    h2 {
      margin-bottom: 500px !important; /* Margin bottom */
      margin-left: 100px; /* Margin left */
      color: green; /* Text color */
    }
  </style>

  <!-- Click category to expand -->
  <h2>Click category to expand</h2>
</body>
</html>
