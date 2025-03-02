# Question Extension Firefox Extension Documentation

This document provides detailed information about the Firefox extension component of the Question Extension.

## Overview

The Question Extension Firefox extension allows users to highlight text in PDF documents opened in Firefox, process that text using a Large Language Model (LLM), and view the results in a popup window. It communicates with a server component that handles the actual LLM processing.

## Features

- Context menu integration for highlighted text in PDFs
- Direct processing of selected text without requiring a specific query
- Formatted display of LLM responses with Markdown support
- Copy-to-clipboard functionality
- Server connection status monitoring

## User Interface Components

### Context Menu

The extension adds a context menu item "Process this text with LLM" that appears when text is highlighted in a PDF document.

### Result Popup

After processing, a popup window displays:
- The original highlighted text
- The LLM's analysis/response
- Buttons to copy the result or close the window

### Toolbar Popup

Clicking the extension icon in the browser toolbar shows:
- Server connection status
- Basic usage instructions
- A button to check server connection

## Installation

### From Firefox Add-ons (recommended)

1. Navigate to the Firefox Add-ons store
2. Search for "PDF LLM Assistant"
3. Click "Add to Firefox"

### Manual Installation (Developer Mode)

1. Download the extension files
2. Open Firefox and navigate to `about:debugging`
3. Click "This Firefox" in the sidebar
4. Click "Load Temporary Add-on"
5. Select the `manifest.json` file in the extension directory

## Usage

1. Open a PDF document in Firefox
2. Highlight text in the document
3. Right-click on the highlighted text
4. Select "Process this text with LLM" from the context menu
5. Wait for processing (a notification will appear)
6. View the response in the popup window
7. Optionally, copy the result using the "Copy Result" button

## Extension Architecture

### Main Components

- **manifest.json**: Extension metadata and permissions
- **background.js**: Handles context menu creation and server communication
- **result.html/js**: Displays the LLM response
- **popup.html/js**: Toolbar popup UI and server status check

### Communication Flow

1. User highlights text and selects the context menu option
2. `background.js` captures the selected text
3. `background.js` sends the text to the server
4. When the response is received, it's stored in browser storage
5. A popup window (`result.html`) is opened to display the result
6. `result.js` formats and displays the response

## Customization

### Modifying the Context Menu Label

In `background.js`, change:
```javascript
browser.contextMenus.create({
  id: "process-text",
  title: "Process this text with LLM", // Change this text
  contexts: ["selection"]
});
```

### Customizing Response Formatting

The formatting of LLM responses is handled in `result.js` by the `formatResponse()` function. You can modify this function to change how different Markdown elements are rendered.

### Changing the Server URL

If you're running the server on a different host or port, update the URL in `background.js`:
```javascript
fetch("http://localhost:5000/query", {  // Change this URL
  // ...
})
```

## Permissions

The extension requires the following permissions:

- **activeTab**: To access the current tab's content
- **contextMenus**: To add items to the context menu
- **storage**: To store and retrieve highlighted text and LLM responses
- **http://localhost:5000/**: To communicate with the local server
- **notifications**: To show processing status notifications

## Troubleshooting

### Common Issues

1. **Context menu item doesn't appear**
   - Ensure you're viewing a PDF document in Firefox
   - Check that text is properly selected

2. **Server connection error**
   - Verify the server is running
   - Check the server URL in the extension
   - Look for CORS issues in the browser console

3. **Formatting issues in responses**
   - The formatting might not work for all responses
   - Check the browser console for JavaScript errors

### Debugging

To debug the extension:

1. Open Firefox and navigate to `about:debugging`
2. Find your extension and click "Inspect"
3. This opens the browser developer tools connected to the extension
4. Check the Console tab for errors or log messages

## Browser Compatibility

- Firefox: Fully supported (version 78+)
- Chrome/Edge: Not supported (uses Firefox-specific APIs)
- Safari: Not supported

## Development

### Building the Extension

To build the extension for distribution:

1. Install web-ext: `npm install -g web-ext`
2. Navigate to the extension directory
3. Run: `web-ext build`
4. The packaged extension (.zip) will be in the `web-ext-artifacts` directory

### Loading for Development

For development and testing:

1. Navigate to `about:debugging` in Firefox
2. Click "This Firefox"
3. Click "Load Temporary Add-on..."
4. Select the `manifest.json` file

### Extension Testing

Test the extension by:
1. Opening a PDF file in Firefox
2. Highlighting different types of text
3. Verifying responses are correctly formatted
4. Testing with the server down to ensure proper error handling