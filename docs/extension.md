# Question Extension Firefox Extension Documentation

## Installation

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

## Permissions

The extension requires the following permissions:

- **activeTab**: To access the current tab's content
- **contextMenus**: To add items to the context menu
- **storage**: To store and retrieve highlighted text and LLM responses
- **http://localhost:5000/**: To communicate with the local server
- **notifications**: To show processing status notifications