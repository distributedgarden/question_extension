// content.js
let selectedText = "";
let dialogOverlay;
let dialogContainer;

// Function that will be checked by the background script
function handlePdfSelection(text) {
  selectedText = text;
  createDialogOverlay();
  return true;
}

// Listen for messages from the background script
browser.runtime.onMessage.addListener((message) => {
  if (message.action === "openDialog") {
    selectedText = message.text;
    createDialogOverlay();
  } else if (message.action === "displayResponse") {
    updateResponseInDialog(message.response);
  }
});

function createDialogOverlay() {
  // Remove existing dialog if any
  removeDialog();
  
  // Create dialog overlay
  dialogOverlay = document.createElement("div");
  dialogOverlay.style.position = "fixed";
  dialogOverlay.style.top = "0";
  dialogOverlay.style.left = "0";
  dialogOverlay.style.width = "100%";
  dialogOverlay.style.height = "100%";
  dialogOverlay.style.backgroundColor = "rgba(0, 0, 0, 0.5)";
  dialogOverlay.style.zIndex = "9999";
  dialogOverlay.style.display = "flex";
  dialogOverlay.style.justifyContent = "center";
  dialogOverlay.style.alignItems = "center";
  
  // Create dialog container
  dialogContainer = document.createElement("div");
  dialogContainer.style.backgroundColor = "white";
  dialogContainer.style.borderRadius = "8px";
  dialogContainer.style.padding = "20px";
  dialogContainer.style.width = "80%";
  dialogContainer.style.maxWidth = "600px";
  dialogContainer.style.maxHeight = "80%";
  dialogContainer.style.overflow = "auto";
  dialogContainer.style.boxShadow = "0 4px 8px rgba(0, 0, 0, 0.2)";
  
  // Create title
  const title = document.createElement("h2");
  title.textContent = "Ask LLM about the highlighted text";
  title.style.margin = "0 0 15px 0";
  
  // Create selected text display
  const textDisplay = document.createElement("div");
  textDisplay.style.backgroundColor = "#f5f5f5";
  textDisplay.style.padding = "10px";
  textDisplay.style.borderRadius = "4px";
  textDisplay.style.marginBottom = "15px";
  textDisplay.style.maxHeight = "100px";
  textDisplay.style.overflow = "auto";
  textDisplay.style.fontSize = "14px";
  textDisplay.textContent = selectedText;
  
  // Create input label
  //const inputLabel = document.createElement("label");
  //inputLabel.textContent = "Your question:";
  //inputLabel.style.display = "block";
  //inputLabel.style.marginBottom = "5px";
  //inputLabel.style.fontWeight = "bold";
  
  // Create input field
  //const input = document.createElement("input");
  //input.type = "text";
  //input.placeholder = "Ask a question about this text...";
  //input.style.width = "100%";
  //input.style.padding = "8px";
  //input.style.boxSizing = "border-box";
  //input.style.marginBottom = "15px";
  //input.style.borderRadius = "4px";
  //input.style.border = "1px solid #ccc";
  
  // Create response container
  const responseContainer = document.createElement("div");
  responseContainer.id = "llm-response-container";
  responseContainer.style.marginTop = "15px";
  responseContainer.style.display = "none";
  
  // Create response title
  const responseTitle = document.createElement("h3");
  responseTitle.textContent = "Response:";
  responseTitle.style.margin = "0 0 5px 0";
  
  // Create response content
  const responseContent = document.createElement("div");
  responseContent.id = "llm-response-content";
  responseContent.style.backgroundColor = "#f0f7ff";
  responseContent.style.padding = "10px";
  responseContent.style.borderRadius = "4px";
  responseContent.style.fontSize = "14px";
  
  // Create loading indicator
  const loadingIndicator = document.createElement("div");
  loadingIndicator.id = "llm-loading-indicator";
  loadingIndicator.textContent = "Loading response...";
  loadingIndicator.style.display = "none";
  loadingIndicator.style.margin = "15px 0";
  loadingIndicator.style.fontStyle = "italic";
  loadingIndicator.style.color = "#666";
  
  // Create buttons container
  const buttonsContainer = document.createElement("div");
  buttonsContainer.style.display = "flex";
  buttonsContainer.style.justifyContent = "flex-end";
  buttonsContainer.style.marginTop = "20px";
  
  // Create ask button
  const askButton = document.createElement("button");
  askButton.textContent = "Ask";
  askButton.style.backgroundColor = "#4285f4";
  askButton.style.color = "white";
  askButton.style.border = "none";
  askButton.style.padding = "8px 16px";
  askButton.style.borderRadius = "4px";
  askButton.style.marginLeft = "10px";
  askButton.style.cursor = "pointer";
  
  // Create close button
  const closeButton = document.createElement("button");
  closeButton.textContent = "Close";
  closeButton.style.backgroundColor = "#f5f5f5";
  closeButton.style.color = "#333";
  closeButton.style.border = "1px solid #ccc";
  closeButton.style.padding = "8px 16px";
  closeButton.style.borderRadius = "4px";
  closeButton.style.cursor = "pointer";
  
  // Add event listeners
  askButton.addEventListener("click", () => {
    const query = input.value.trim();
    if (query) {
      // Show loading indicator
      loadingIndicator.style.display = "block";
      responseContainer.style.display = "none";
      
      // Send message to background script
      browser.runtime.sendMessage({
        action: "queryLLM",
        text: selectedText,
        query: query
      });
    }
  });
  
  closeButton.addEventListener("click", removeDialog);
  
  // Add input event for Enter key
  input.addEventListener("keyup", (event) => {
    if (event.key === "Enter") {
      askButton.click();
    }
  });
  
  // Close when clicking outside the dialog
  dialogOverlay.addEventListener("click", (event) => {
    if (event.target === dialogOverlay) {
      removeDialog();
    }
  });
  
  // Assemble the dialog
  responseContainer.appendChild(responseTitle);
  responseContainer.appendChild(responseContent);
  
  buttonsContainer.appendChild(closeButton);
  buttonsContainer.appendChild(askButton);
  
  dialogContainer.appendChild(title);
  dialogContainer.appendChild(textDisplay);
  //dialogContainer.appendChild(inputLabel);
  //dialogContainer.appendChild(input);
  dialogContainer.appendChild(loadingIndicator);
  dialogContainer.appendChild(responseContainer);
  dialogContainer.appendChild(buttonsContainer);
  
  dialogOverlay.appendChild(dialogContainer);
  document.body.appendChild(dialogOverlay);
  
  // Focus the input field
  input.focus();
}

function updateResponseInDialog(response) {
  const loadingIndicator = document.getElementById("llm-loading-indicator");
  const responseContainer = document.getElementById("llm-response-container");
  const responseContent = document.getElementById("llm-response-content");
  
  if (loadingIndicator && responseContainer && responseContent) {
    loadingIndicator.style.display = "none";
    responseContainer.style.display = "block";
    responseContent.textContent = response;
  }
}

function removeDialog() {
  if (dialogOverlay && dialogOverlay.parentNode) {
    dialogOverlay.parentNode.removeChild(dialogOverlay);
  }
}

// Expose the function to global scope to make it detectable
window.handlePdfSelection = handlePdfSelection;