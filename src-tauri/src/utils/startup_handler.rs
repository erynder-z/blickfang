use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State};

// State for caching paths that arrive before the frontend is ready.
#[derive(Default)]
pub struct OpenedPathsState {
    pub paths: Arc<Mutex<Vec<String>>>,
}

// State to track if the frontend has signaled that it is ready.
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
        let paths_to_send = guard.clone();
        guard.clear();
        if let Err(_) = app.emit("image-source", paths_to_send) {}
    }
}

pub fn handle_run_event(app_handle: &AppHandle, event: &RunEvent) {
    if let RunEvent::Opened { urls, .. } = event {
        let app_ready_state = app_handle.state::<AppReady>();

        let paths_to_process: Vec<String> = urls
            .iter()
            .filter_map(|url| {
                url.to_file_path()
                    .ok()
                    .and_then(|path| path.to_str().map(String::from))
            })
            .collect();

        if !paths_to_process.is_empty() {
            // Check if the frontend is ready.
            if app_ready_state.0.load(Ordering::Relaxed) {
                if let Err(_) = app_handle.emit("image-source", paths_to_process) {}
            } else {
                // If not ready, cache the paths.
                let opened_paths_state = app_handle.state::<OpenedPathsState>();
                let mut guard = opened_paths_state.paths.lock().unwrap();
                for path in paths_to_process {
                    guard.push(path);
                }
            }
        }
    }
}