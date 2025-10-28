use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State};

#[derive(Default)]
pub struct OpenedPathsState {
    pub paths: Arc<Mutex<Vec<String>>>,
}

#[derive(Default)]
pub struct AppReady(pub AtomicBool);

#[tauri::command]
pub fn frontend_is_ready(
    opened_paths_state: State<OpenedPathsState>,
    app_ready_state: State<AppReady>,
    app: AppHandle,
) {
    app_ready_state.0.store(true, Ordering::Relaxed);
    let mut guard = opened_paths_state.paths.lock().unwrap();
    if !guard.is_empty() {
        let paths_to_send: Vec<String> = guard.drain(..).collect();
        if let Err(e) = app.emit("image-source", paths_to_send) {
            eprintln!("Failed to emit 'image-source' event: {}", e);
        }
    }
}

pub fn handle_run_event(app_handle: &AppHandle, event: &RunEvent) {
    #[cfg(target_os = "macos")]
    if let RunEvent::Opened { urls, .. } = event {
        handle_opened_paths(
            app_handle,
            urls.iter()
                .filter_map(|url| {
                    url.to_file_path()
                        .ok()
                        .and_then(|p| p.to_str().map(String::from))
                })
                .collect(),
        );
    }

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Skip the first argument (the executable path)
    if let RunEvent::Ready = event {
        let args: Vec<String> = std::env::args().skip(1).collect();
        if !args.is_empty() {
            handle_opened_paths(app_handle, args);
        }
    }
}

fn handle_opened_paths(app_handle: &AppHandle, paths: Vec<String>) {
    let app_ready_state = app_handle.state::<AppReady>();

    if app_ready_state.0.load(Ordering::Relaxed) {
        if let Err(e) = app_handle.emit("image-source", paths) {
            eprintln!("Failed to emit 'image-source' event: {}", e);
        }
    } else {
        let opened_paths_state = app_handle.state::<OpenedPathsState>();
        let mut guard = opened_paths_state.paths.lock().unwrap();
        guard.extend(paths);
    }
}
