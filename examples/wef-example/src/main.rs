use std::sync::mpsc;
use wef::{Browser, BrowserHandler, Settings, FuncRegistry};

struct SimpleHandler;

impl BrowserHandler for SimpleHandler {
    fn on_context_created(&mut self, _frame: &wef::Frame) {}
    
    fn on_before_browse(&mut self, _frame: &wef::Frame, _request: &wef::ffi::CefRequest, _user_gesture: bool, _is_redirect: bool) -> bool {
        false
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple wef example showing basic browser creation
    println!("Starting wef example...");
    
    let settings = Settings::new()
        .with_log_severity(wef::settings::LogSeverity::Info)
        .with_cache_path(std::env::temp_dir().join("wef-example-cache"));
    
    // Create a function registry for JavaScript bridge
    let func_registry = FuncRegistry::builder()
        .register("hello", || "Hello from Rust!".to_string())
        .register("add", |a: i32, b: i32| a + b)
        .build();
    
    wef::launch(settings, || {
        println!("CEF initialized successfully");
        
        // Create a browser instance
        let handler = SimpleHandler;
        let browser = Browser::builder()
            .with_url("data:text/html,<html><body><h1>Hello from Wef!</h1><script>console.log('Wef browser started');</script></body></html>")
            .with_size(800, 600)
            .with_handler(handler)
            .with_func_registry(func_registry)
            .build();
            
        if let Ok(_browser) = browser {
            println!("Browser created successfully!");
            
            // In a real application, you would handle events and keep the browser running
            // For this example, we'll just demonstrate that the browser can be created
            std::thread::sleep(std::time::Duration::from_secs(2));
        } else {
            eprintln!("Failed to create browser");
        }
        
        println!("Wef example completed");
    });
    
    Ok(())
}
