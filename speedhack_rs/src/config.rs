use eyre::Context;
use rust_hooking_utils::raw_input::virtual_keys::VirtualKey;
use std::path::Path;
use std::time::Duration;

pub const CONFIG_FILE_NAME: &str = "speedhack_config.json";

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct SpeedhackConfig {
    pub console: bool,
    pub wait_with_hook: Option<Duration>,
    pub reload_config_keys: Option<Vec<VirtualKey>>,
    pub startup_state: Option<StartupConfig>,
    pub speed_states: Vec<SpeedStateConfig>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StartupConfig {
    pub speed: f64,
    pub duration: Option<Duration>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct SpeedStateConfig {
    pub keys: Vec<VirtualKey>,
    pub speed: f64,
    pub is_toggle: bool,
}

// === BAGIAN INI YANG KITA MODIFIKASI ===
impl Default for SpeedhackConfig {
    fn default() -> Self {
        Self {
            // Aktifkan console by default agar bisa debug crash
            console: true, 
            // Tunggu 2 detik (lebih aman) sebelum hook agar nmcogame.dll siap
            wait_with_hook: Some(Duration::from_millis(2000)), 
            
            // Tombol Reload Config: CTRL + R
            reload_config_keys: Some(vec![VirtualKey::VK_CONTROL, VirtualKey::VK_R]),
            
            startup_state: None,
            
            // Logic Speedhack Default
            speed_states: vec![
                // State 1: Tekan F5 untuk Toggle Speed 3.0x (Sama seperti C++ kita)
                SpeedStateConfig {
                    keys: vec![VirtualKey::VK_F5],
                    speed: 3.0,
                    is_toggle: true,
                },
                // State 2: Tahan F6 untuk Turbo Speed 10.0x
                SpeedStateConfig {
                    keys: vec![VirtualKey::VK_F6],
                    speed: 10.0,
                    is_toggle: false,
                }
            ],
        }
    }
}

pub fn create_initial_config(directory: impl AsRef<Path>) -> eyre::Result<()> {
    let default_conf = SpeedhackConfig::default();
    let path = directory.as_ref().join(CONFIG_FILE_NAME);

    // Selalu buat file config baru jika belum ada
    if !path.exists() {
        let mut file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(&mut file, &default_conf)?;
    }

    Ok(())
}

pub fn load_config(directory: impl AsRef<Path>) -> eyre::Result<SpeedhackConfig> {
    let path = directory.as_ref().join(CONFIG_FILE_NAME);
    
    // Jika file config tidak ada, gunakan default (F5/F6) tanpa error
    if !path.exists() {
        return Ok(SpeedhackConfig::default());
    }

    let file = std::fs::read(path)?;
    let conf = serde_json::from_slice(&file).context("Failed to read config file, is it valid?")?;

    validate_config(&conf)?;

    Ok(conf)
}

fn validate_config(config: &SpeedhackConfig) -> eyre::Result<()> {
    let mut errors = Vec::new();

    for state in &config.speed_states {
        if state.speed <= 0. {
            errors.push(format!(
                "Speed for every speed state needs to be more than `0`, found `{:?}",
                state.speed
            ))
        }
    }

    let error = errors.join("\n");

    if error.is_empty() {
        Ok(())
    } else {
        Err(eyre::Error::msg(error))
    }
}
