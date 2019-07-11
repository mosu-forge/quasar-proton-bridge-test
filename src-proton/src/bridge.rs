use web_view::*;

/*
The intention here was to make a class that held a reference to the webview
so that it could have a send method and also listen for messages internally
and emit an event to main.rs. This may not be the correct approach for Rust,
but is similar to how it is done in app-extension-electron-security.

Below code also does not work, need to read rust docs more to get familiar
with passing a reference to a struct.
*/

/*
pub struct Bridge<'a, T> {
    pub webview: &'a mut WebView<'a, T>
}

impl<'a, WebView> Bridge<'a, WebView> {
    pub fn new(webview: &mut WebView) -> Bridge<'a, WebView> {
        Bridge{
            webview: webview
        }
    }
}
*/
