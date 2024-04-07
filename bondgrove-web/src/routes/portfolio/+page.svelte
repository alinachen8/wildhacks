<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Portfolio</title>
  <!-- Load charting library -->
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body>
  <h1>Portfolio Overview</h1>

  <!-- Pie chart showing diversity -->
  <canvas id="diversityChart" width="400" height="400"></canvas>

  <!-- Bar chart showing number of bonds and total money invested -->
  <canvas id="investmentChart" width="400" height="400"></canvas>

  <script>
    // Data for the pie chart (example)
    const diversityData = {
      labels: ['Stocks', 'Bonds', 'Real Estate', 'Cash'],
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
</body>
</html>
