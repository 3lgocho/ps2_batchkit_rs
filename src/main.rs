mod core;
mod ps2; // Conectamos la carpeta ps2

#[tokio::main]
async fn main() {
    println!("🚀 Iniciando Batchkit Manager Universal...\n");

    // Buscamos nuestro ejecutable local
    let bin_path = core::downloader::get_bin_dir().join("hdl_dump");
    let iso_path = "/home/andres/Downloads/ICO (Europe).iso"; // Tu ruta real

    if bin_path.exists() {
        println!("💿 Analizando ISO con el nuevo Wrapper...");
        
        // Llamamos a nuestra función inteligente
        match ps2::hdl_dump_wrap::obtener_info_iso(iso_path, &bin_path) {
            Ok(info_juego) => {
                println!("✅ ¡Éxito! Datos extraídos:");
                println!("   - ID del Juego:   {}", info_juego.id);
                println!("   - Nombre Interno: '{}'", info_juego.nombre_interno);
                println!("   - Tamaño en KB:   {}", info_juego.tamano_kb);
                
                // ¡Aquí es donde la programación en Rust brilla!
                // Ahora puedes usar `info_juego.id` para renombrar particiones automáticamente
                let nombre_particion = format!("PP.HDL.{}", info_juego.id);
                println!("   👉 Próxima partición a crear: {}", nombre_particion);
            },
            Err(e) => eprintln!("❌ Error al leer ISO: {}", e),
        }
    } else {
        eprintln!("❌ No se encontró hdl_dump en bin/");
    }
}