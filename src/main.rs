mod core;
use std::process::Command;

#[tokio::main]
async fn main() {
    println!("🚀 Verificando el Taller Local de Binarios...\n");

    let herramientas = vec!["hdl_dump", "pfsshell", "cue2pops"];

    for bin_name in herramientas {
        let bin_path = core::downloader::get_bin_dir().join(bin_name);

        if bin_path.exists() {
            println!("--------------------------------------------------");
            println!("🔧 Testeando: {}", bin_name);
            
            // Ejecutamos la herramienta sin argumentos para forzar que escupa su menú de ayuda
            let output = Command::new(&bin_path)
                .output() 
                .expect("Falló la ejecución del binario");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Queremos ver al menos la primera línea del menú de ayuda para confirmar
            let salida_completa = format!("{}{}", stdout, stderr);
            let primera_linea = salida_completa.lines().next().unwrap_or("Sin salida");
            
            println!("✅ OK! Responde: {}", primera_linea);
        } else {
            println!("❌ ERROR: No se encontró el binario en {:?}", bin_path);
        }
    }
    println!("--------------------------------------------------");
}