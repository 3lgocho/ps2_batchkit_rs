use std::process::Command;
use std::path::PathBuf;

// Definimos una 'Estructura' para guardar los datos del juego de forma ordenada
#[derive(Debug)]
pub struct GameInfo {
    pub id: String,
    pub nombre_interno: String,
    pub tamano_kb: u64,
}

/// Ejecuta 'hdl_dump cdvd_info' y parsea el resultado
pub fn obtener_info_iso(iso_path: &str, bin_path: &PathBuf) -> Result<GameInfo, String> {
    let output = Command::new(bin_path)
        .arg("cdvd_info")
        .arg(iso_path)
        .output()
        .map_err(|e| format!("Falló al ejecutar hdl_dump: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Si hay error en la lectura (ej. ISO corrupta o no es de PS2)
    if stdout.is_empty() || stdout.contains("error") {
        return Err("La ISO no es válida o hdl_dump no pudo leerla.".to_string());
    }

    // El formato esperado es: "SCES_507.60" "Nombre"  886432KB
    // Al separar por comillas ("), obtenemos:
    // parts[0] = (vacío)
    // parts[1] = SCES_507.60
    // parts[2] =  (espacio)
    // parts[3] = Nombre interno (suele estar vacío en muchas ISOs)
    // parts[4] =   886432KB
    let parts: Vec<&str> = stdout.split('"').collect();

    if parts.len() >= 5 {
        let id = parts[1].to_string();
        let nombre_interno = parts[3].to_string();
        
        // Limpiamos la parte del tamaño quitando "KB" y espacios
        let size_str = parts[4].replace("KB", "").trim().to_string();
        // Convertimos el texto a número (u64)
        let tamano_kb = size_str.parse::<u64>().unwrap_or(0);

        Ok(GameInfo {
            id,
            nombre_interno,
            tamano_kb,
        })
    } else {
        Err(format!("Formato de salida inesperado: {}", stdout))
    }
}