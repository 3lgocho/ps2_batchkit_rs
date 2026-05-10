use std::env;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt; // Necesario para dar permisos +x en Linux

// Definimos los repositorios oficiales de donde haremos el pull
const REPO_HDL_DUMP: &str = "ps2homebrew/hdl-dump";
const REPO_PFSSHELL: &str = "ps2homebrew/pfsshell";
const REPO_CUE2POPS: &str = "israpps/cue2pops";

/// Obtiene la ruta de la carpeta 'bin' donde se guardarán las dependencias
pub fn get_bin_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ps2homebrew", "BatchKitManager") {
        let bin_dir = proj_dirs.data_local_dir().join("bin");
        // Crea la carpeta (ej. %APPDATA%\BatchKitManager\data\bin) si no existe
        if !bin_dir.exists() {
            fs::create_dir_all(&bin_dir).expect("No se pudo crear el directorio de binarios");
        }
        bin_dir
    } else {
        // Fallback a la carpeta actual si falla la detección del OS
        PathBuf::from("./bin")
    }
}

/// Verifica si un binario específico ya está descargado
pub fn is_binary_installed(binary_name: &str) -> bool {
    let mut path = get_bin_dir().join(binary_name);
    
    // Si estamos en Windows, añadimos .exe
    if env::consts::OS == "windows" {
        path.set_extension("exe");
    }
    
    path.exists()
}

/// Descarga la última versión (Release) desde GitHub
pub async fn download_dependency(client: &Client, repo: &str, binary_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Falta {}. Consultando última versión en {}...", binary_name, repo);
    
    // 1. Consultar la API de GitHub para obtener el último Release
    // Nota: GitHub requiere un User-Agent válido
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let response = client.get(&api_url)
        .header("User-Agent", "PFS-BatchKit-Manager-Rust")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // 2. Aquí buscarías en `response["assets"]` el archivo que coincida con tu SO (windows/linux)
    // Para simplificar, asumiremos que ya obtuviste la URL de descarga directa (download_url)
    // let download_url = extraer_url_por_sistema_operativo(&response);
    
    // URL simulada para el ejemplo
    let download_url = "URL_DEL_BINARIO_AQUI"; 
    
    // 3. Descargar el archivo
    println!("Descargando {}...", binary_name);
    let bin_data = client.get(download_url).send().await?.bytes().await?;
    
    // 4. Guardar en disco
    let mut file_path = get_bin_dir().join(binary_name);
    if env::consts::OS == "windows" {
        file_path.set_extension("exe");
    }
    
    let mut file = File::create(&file_path)?;
    file.write_all(&bin_data)?;
    
    // 5. Asignar permisos de ejecución si es Linux/Mac
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&file_path)?.permissions();
        perms.set_mode(0o755); // Permisos rwxr-xr-x (+x)
        fs::set_permissions(&file_path, perms)?;
        println!("Permisos de ejecución (+x) asignados a {}", binary_name);
    }
    
    println!("{} instalado correctamente en {:?}", binary_name, file_path);
    Ok(())
}