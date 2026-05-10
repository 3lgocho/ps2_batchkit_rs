use std::env;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;
#[allow(dead_code)]

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

// Rutas de los repositorios
const REPO_HDL_DUMP: &str = "ps2homebrew/hdl-dump";
const REPO_PFSSHELL: &str = "ps2homebrew/pfsshell";
const REPO_CUE2POPS: &str = "israpps/cue2pops";

pub fn get_bin_dir() -> PathBuf {
    let bin_dir = PathBuf::from("./bin");
    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir).expect("No se pudo crear la carpeta de binarios");
    }
    bin_dir
}

pub async fn download_latest_release(client: &Client, repo: &str, binary_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("🔍 Buscando la última versión de {} en GitHub...", binary_name);
    
    // 1. Consultar la API de GitHub
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let response = client.get(&api_url)
        .header("User-Agent", "PFS-BatchKit-Manager-Rust")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // 2. Determinar la plataforma actual
    let target_os = env::consts::OS; // "windows" o "linux"
    
    // 3. Buscar el enlace de descarga en los 'assets'
    let mut download_url = String::new();
    let mut actual_filename = String::new();

    if let Some(assets) = response["assets"].as_array() {
        for asset in assets {
            let name = asset["name"].as_str().unwrap_or("").to_lowercase();
            // Filtramos por nuestro sistema operativo de forma básica
            if (target_os == "windows" && name.contains("win")) || 
               (target_os == "linux" && name.contains("linux")) {
                download_url = asset["browser_download_url"].as_str().unwrap_or("").to_string();
                actual_filename = asset["name"].as_str().unwrap_or("").to_string();
                break;
            }
        }
    }

    if download_url.is_empty() {
        return Err(format!("No se encontró un binario compatible para {} en el repositorio {}", target_os, repo).into());
    }

    // 4. Descargar el archivo
    println!("⬇️  Descargando {}...", actual_filename);
    let bin_data = client.get(&download_url).send().await?.bytes().await?;
    
    // 5. Guardar en disco
    let mut file_path = get_bin_dir().join(binary_name);
    if target_os == "windows" {
        file_path.set_extension("exe");
    }
    
    let mut file = File::create(&file_path)?;
    file.write_all(&bin_data)?;
    
    // 6. Permisos de ejecución para Linux/Mac
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&file_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&file_path, perms)?;
    }
    
    println!("✅ {} descargado e instalado en {:?}", binary_name, file_path);
    Ok(file_path)
}