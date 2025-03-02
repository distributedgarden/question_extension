// background.js
browser.contextMenus.create({
  id: "ask-llm",
  title: "Ask LLM about this text",
  contexts: ["selection"]
});

// Handle context menu click
browser.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === "ask-llm") {
    // Get the selected text
    const selectedText = info.selectionText;
    
    // Save the selected text to storage for the popup to access
    browser.storage.local.set({
      selectedText: selectedText
    }).then(() => {
      // Open popup window with the query UI
      browser.windows.create({
        url: browser.runtime.getURL("popup/query.html"),
        type: "popup",
        width: 600,
        height: 500
      });
    });
  }
});

// Handle messages from popup
browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.action === "queryLLM") {
    // Send the query to the Flask server
    fetch("http://localhost:5000/query", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        text: message.text,
        //query: message.query
      })
    })
    .then(response => response.json())
    .then(data => {
      // Send response back to the popup
      sendResponse({ 
        success: true, 
        response: data.response 
      });
    })
    .catch(error => {
      console.error("Error:", error);
      sendResponse({ 
        success: false, 
        response: "Error connecting to LLM server. Please make sure the Flask server is running." 
      });
    });
    
    return true; // Required for async responses
  }
});