mod core;
use core::downloader;

#[tokio::main]
async fn main() {
    println!("Iniciando Batchkit Manager Universal...\n");

    let client = reqwest::Client::new();
    let dependencias = vec![
        ("hdl_dump", "ps2homebrew/hdl-dump"),
        ("pfsshell", "ps2homebrew/pfsshell"),
        ("cue2pops", "israpps/cue2pops"),
    ];

    // Chequeo de dependencias
    for (bin, repo) in dependencias {
        if !downloader::is_binary_installed(bin) {
            match downloader::download_dependency(&client, repo, bin).await {
                Ok(_) => {},
                Err(e) => eprintln!("Error fatal descargando {}: {}", bin, e),
            }
        } else {
            println!("✅ {} ya está instalado.", bin);
        }
    }

    println!("\nTodas las dependencias están listas.");
    println!("Lanzando la interfaz principal (TUI)...");
    
    // Aquí llamarías a la lógica de tus menús (src/tui/menus.rs)
    // tui::menus::mostrar_menu_principal();
}