// popup.js
document.addEventListener("DOMContentLoaded", function() {
  const statusDiv = document.getElementById("server-status");
  const checkButton = document.getElementById("check-connection");
  
  // Check server connection when popup opens
  checkServerConnection();
  
  // Add event listener for check button
  checkButton.addEventListener("click", checkServerConnection);
  
  function checkServerConnection() {
    statusDiv.textContent = "Server status: Checking...";
    statusDiv.className = "status";
    
    fetch("http://localhost:5000/status", {
      method: "GET"
    })
    .then(response => {
      if (response.ok) {
        return response.json();
      }
      throw new Error("Server not responding");
    })
    .then(data => {
      statusDiv.textContent = "Server status: Connected";
      statusDiv.className = "status connected";
    })
    .catch(error => {
      statusDiv.textContent = "Server status: Disconnected";
      statusDiv.className = "status disconnected";
    });
  }
});