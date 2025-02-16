import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export function openWindow(child_window: ChildWindow, disable_parent: Function) {
    const webview = new WebviewWindow(child_window.name, {url: child_window.url, title: child_window.title});

    disable_parent();
    
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once('tauri://created', function (e) {
      if (child_window.created_callback) {
        child_window.created_callback(e);
      }
    })
    webview.once('tauri://error', function (e) {
      if (child_window.error_callback) {
        child_window.error_callback(e);
      }
    })
    webview.once('tauri://closed', function (e) {
      if (child_window.close_callback) {
        child_window.close_callback(e);
      }
    })
    webview.once("tauri://close-requested", function (e) {
      if (child_window.close_callback) {
        child_window.close_callback(e);
      }
    })
  }