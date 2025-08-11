use std::sync::Arc;
use tauri::{State, Manager};
use serde_json::Value;
use std::process::{Command, Stdio};
use tokio::sync::Mutex;

// State to store sidecar information
#[derive(Default)]
struct SidecarState {
    python_port: Arc<Mutex<Option<u16>>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_python_sidecar(
    app_handle: tauri::AppHandle,
    state: State<'_, SidecarState>,
) -> Result<u16, String> {
    // Check if we already have a running port
    {
        let port_lock = state.python_port.lock().await;
        if let Some(port) = *port_lock {
            // Test if server is still running
            let url = format!("http://localhost:{}/health", port);
            if let Ok(response) = reqwest::get(&url).await {
                if response.status().is_success() {
                    return Ok(port);
                }
            }
        }
    } // Release lock here
    
    // Find available port
    let port = find_available_port().unwrap_or(8765);
    
    // Get the Python script path
    let (python_script, work_dir) = if cfg!(debug_assertions) {
        // In development mode, use src-tauri/python/main.py
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let script_path = current_dir.join("python").join("main.py");
        let working_dir = current_dir;
        (script_path, working_dir)
    } else {
        // In release mode, use bundled resources
        let resource_dir = app_handle
            .path()
            .resource_dir()
            .map_err(|e| format!("Failed to get resource directory: {}", e))?;
        let script_path = resource_dir.join("python").join("main.py");
        (script_path, resource_dir)
    };
    
    println!("Looking for Python script at: {}", python_script.display());
    println!("Working directory: {}", work_dir.display());
    
    if !python_script.exists() {
        return Err(format!("Python script not found at: {}", python_script.display()));
    }
    
    // Try to find Python executable
    let python_cmd = get_python_command(&work_dir)?;
    
    println!("Using Python command: {}", python_cmd);
    
    let _child = Command::new(&python_cmd)
        .arg(python_script.to_string_lossy().as_ref())
        .arg(port.to_string())
        .current_dir(&work_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start Python server with '{}': {}", python_cmd, e))?;
    
    // Wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    
    // Test server is running
    let url = format!("http://localhost:{}/health", port);
    match reqwest::get(&url).await {
        Ok(response) if response.status().is_success() => {
            // Store the port after confirming server is running
            let mut port_lock = state.python_port.lock().await;
            *port_lock = Some(port);
            Ok(port)
        },
        _ => Err("Failed to start Python server or server not responding".to_string())
    }
}

fn get_python_command(resource_dir: &std::path::Path) -> Result<String, String> {
    // Try bundled Python first
    if cfg!(target_os = "windows") {
        let bundled_python = resource_dir.join("python").join("python-dist").join("python.exe");
        if bundled_python.exists() {
            return Ok(bundled_python.to_string_lossy().to_string());
        }
        // Fallback to system Python
        Ok("python".to_string())
    } else {
        let bundled_python = resource_dir.join("python").join("python-dist").join("python");
        if bundled_python.exists() {
            return Ok(bundled_python.to_string_lossy().to_string());
        }
        // Fallback to system Python
        Ok("python3".to_string())
    }
}

#[tauri::command]
async fn get_trading_history(
    app_handle: tauri::AppHandle,
    state: State<'_, SidecarState>,
) -> Result<Value, String> {
    // Ensure Python sidecar is running
    let port = start_python_sidecar(app_handle, state).await?;
    
    // Make HTTP request to Python sidecar
    let url = format!("http://localhost:{}/trading-history", port);
    
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP request failed with status: {}", response.status()));
    }
    
    let json_result: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;
    
    Ok(json_result)
}

fn find_available_port() -> Option<u16> {
    use std::net::TcpListener;
    
    for port in 8765..8800 {
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return Some(port);
        }
    }
    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(SidecarState::default())
        .invoke_handler(tauri::generate_handler![
            greet, 
            start_python_sidecar, 
            get_trading_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
