#[tauri::command]
fn get_loaded_ssh_agent_keys() -> Result<Vec<String>, String> {
    let output = std::process::Command::new("ssh-add")
        .arg("-l")
        .output();
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let lines: Vec<String> = stdout.lines().map(|l| l.to_string()).collect();
                Ok(lines)
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                Err(stderr)
            }
        }
        Err(e) => Err(format!("Failed to run ssh-add -l: {}", e)),
    }
}

#[tauri::command]
fn is_ssh_agent_running() -> bool {
    match std::env::var("SSH_AUTH_SOCK") {
        Ok(val) => !val.trim().is_empty(),
        Err(_) => false,
    }
}
use std::fs;
use std::process::Command;



#[tauri::command]
fn get_ssh_keys() -> Result<Vec<String>, String> {
    println!("Invoked list_ssh_keys method");
    let ssh_dir = dirs::home_dir()
        .map(|mut p| {
            p.push(".ssh");
            p
        })
        .ok_or("Could not find home directory")?;
    println!("Post ssh dir: {:?}", ssh_dir);

    let entries = fs::read_dir(&ssh_dir)
        .map_err(|_| format!("Could not read directory: {:?}", ssh_dir))?;

    let mut key_outputs = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("pub") {
                let output = Command::new("ssh-keygen")
                    .arg("-lv")
                    .arg("-f")
                    .arg(&path)
                    .output();
                match output {
                    Ok(out) => {
                        if out.status.success() {
                            let result = String::from_utf8_lossy(&out.stdout).to_string();
                            key_outputs.push(result);
                        } else {
                            let err = String::from_utf8_lossy(&out.stderr).to_string();
                            key_outputs.push(format!("Error for {}: {}", path.display(), err));
                        }
                    }
                    Err(e) => {
                        key_outputs.push(format!("Failed to run ssh-keygen for {}: {}", path.display(), e));
                    }
                }
            }
        }
    }

    println!("Post pub_keys: {:?}", key_outputs);

    Ok(key_outputs)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_ssh_keys, is_ssh_agent_running, get_loaded_ssh_agent_keys])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
