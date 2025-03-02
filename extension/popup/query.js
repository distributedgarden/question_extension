// query.js
document.addEventListener("DOMContentLoaded", function() {
  const selectedTextElement = document.getElementById("selectedText");
  //const questionInput = document.getElementById("questionInput");
  const askButton = document.getElementById("askButton");
  const closeButton = document.getElementById("closeButton");
  const loadingIndicator = document.getElementById("loadingIndicator");
  const responseContainer = document.getElementById("responseContainer");
  const responseContent = document.getElementById("responseContent");
  
  let selectedText = "";
  
  // Get the selected text from storage
  browser.storage.local.get("selectedText").then(data => {
    if (data.selectedText) {
      selectedText = data.selectedText;
      selectedTextElement.textContent = selectedText;
    } else {
      selectedTextElement.textContent = "No text selected";
    }
  });
  
  // Add event listeners
  askButton.addEventListener("click", askQuestion);
  closeButton.addEventListener("click", closeWindow);
  questionInput.addEventListener("keyup", function(event) {
    if (event.key === "Enter") {
      askQuestion();
    }
  });
  
  // Focus the input field
  questionInput.focus();
  
  function askQuestion() {
    //const query = questionInput.value.trim();
    if (selectedText) {
      // Show loading indicator
      loadingIndicator.style.display = "block";
      responseContainer.style.display = "none";
      responseContent.classList.remove("error");
      
      // Send the query to the background script
      browser.runtime.sendMessage({
        action: "queryLLM",
        text: selectedText,
        //query: query
      }).then(response => {
        // Hide loading indicator
        loadingIndicator.style.display = "none";
        responseContainer.style.display = "block";
        
        // Display the response
        if (response && response.success) {
          responseContent.textContent = response.response;
        } else {
          responseContent.classList.add("error");
          responseContent.textContent = response.response || "Error: No response received";
        }
      }).catch(error => {
        loadingIndicator.style.display = "none";
        responseContainer.style.display = "block";
        responseContent.classList.add("error");
        responseContent.textContent = "Error: " + error.message;
      });
    }
  }
  
  function closeWindow() {
    window.close();
  }
});