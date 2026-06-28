// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bridge;
mod stats;

use bridge::events::{
    do_send_connection_status, listen_connect_elm, listen_decode_vin, listen_send_ports,
};
use obdium::{
    vin::{vpic_db_path, APP_DATA_DIR},
    OBD,
};
use stats::{
    critical_frequency_calls, custom_pid_calls, frequent_calls, high_frequency_calls,
    less_frequent_calls, once_calls, oxygen_sensors,
};

use std::{
    fs::File,
    io::{copy, BufReader, BufWriter, Error},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::sleep,
    time::Duration,
};
use tauri::{async_runtime::spawn, Emitter, Listener, Manager, WebviewWindow};

use crate::bridge::{events::listen_track_custom_pid, FrontendNotification};

use xz2::read::XzDecoder;

/// Loads the `vpic.sqlite` file by decoding the
/// `vpic.sqlite.xz` file.
///
/// This alternative was created to avoid having to break the bank
/// paying for Git LFS to store a single, static 1.3GB file.
///
/// The VPIC sqlite database is critical for receiving information from a VIN
/// in the VIN decoder.
fn try_load_vpic_database(
    window: &Arc<WebviewWindow>,
    db_path: PathBuf,
    xz_path: PathBuf,
) -> Result<(), Error> {
    const EXPECTED_VPIC_SQLITE_FILE_SIZE_BYTES: u64 = 1453146112;

    if let Ok(vpic_sqlite) = File::open(&db_path) {
        // vpic sqlite file exists. verify the size
        if let Ok(metadata) = vpic_sqlite.metadata() {
            if metadata.len() == EXPECTED_VPIC_SQLITE_FILE_SIZE_BYTES {
                println!("[LOAD_VPIC_DATABASE] `vpic.sqlite` already loaded.");
                return Ok(());
            }
        }
    }

    println!(
        "[LOAD_VPIC_DATABASE] Loading `vpic.sqlite` - decompressing data from `vpic.sqlite.xz"
    );
    let _ = window.emit("display-notification", FrontendNotification {
        title: "Base de Datos VIN (VPIC)",
        description: "Descomprimiendo base de datos VIN (VPIC) por primera y única vez - espere antes de usar el decodificador VIN."
    });

    // io files
    let input = File::open(xz_path)?;
    let output = File::create(db_path)?;

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut decompressor = XzDecoder::new(reader);
    if copy(&mut decompressor, &mut writer).is_ok() {
        println!("[LOAD_VPIC_DATABASE] Successfully decompressed and loaded `vpic.sqlite.xz` into `vpic.sqlite`");
        let _ = window.emit("display-notification", FrontendNotification {
            title: "Base de Datos VIN (VPIC)",
            description: "Base de datos VIN (VPIC) cargada correctamente - ya puede decodificar VINs."
        });
    } else {
        let _ = window.emit("display-notification", FrontendNotification {
            title: "Base de Datos VIN (VPIC)",
            description: "Error al cargar la base de datos VIN - el decodificador VIN puede no funcionar correctamente. Reinicie la aplicación o reporte el problema en el repositorio."
        });
    }

    Ok(())
}

fn track_data(window: &Arc<WebviewWindow>, obd: &Arc<Mutex<OBD>>) {
    critical_frequency_calls(window, obd);
    high_frequency_calls(window, obd);
    frequent_calls(window, obd);
    less_frequent_calls(window, obd);
    oxygen_sensors(window, obd);
    once_calls(window, obd);
    custom_pid_calls(window, obd);
}

fn connect_obd(window: &WebviewWindow, port: String, baud_rate: u32, protocol: u8) -> Option<OBD> {
    // Try to connect obd
    let mut obd = OBD::new();

    match obd.connect(&port, baud_rate, protocol) {
        Ok(()) => {
            let band = obd.serial_port_baud_rate().unwrap_or_default();
            let port = obd.serial_port_name().unwrap_or_default();

            do_send_connection_status(
                window,
                &obd,
                format!("Connected to port {port} on {band} baud"),
                true,
            );

            Some(obd)
        }
        Err(error) => {
            do_send_connection_status(
                window,
                &obd,
                format!("Failed to connect. Error: {error}"),
                false,
            );

            None
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let frontend_ready = Arc::new(AtomicBool::new(false));

            // load app data dir for the first time
            let app_data_dir = app.path().app_data_dir()?;
            APP_DATA_DIR.get_or_init(|| app_data_dir.clone());
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = vpic_db_path().unwrap();
            let xz_path = app
                .path()
                .resolve("data/vpic.sqlite.xz", tauri::path::BaseDirectory::Resource)
                .unwrap();

            let code_desc_src = app
                .path()
                .resolve("data/code-descriptions.sqlite", tauri::path::BaseDirectory::Resource)
                .unwrap();
            let mode22_pids_src = app
                .path()
                .resolve("data/model-pids.sqlite", tauri::path::BaseDirectory::Resource)
                .unwrap();

            spawn(async move {
                // Detect when the frontend is loaded
                let frontend_ready_listener = Arc::clone(&frontend_ready);
                window.listen("frontend-loaded", move |_| {
                    frontend_ready_listener.store(true, Ordering::SeqCst);
                });

                let window_arc = Arc::new(window);

                while !frontend_ready.load(Ordering::SeqCst) {
                    sleep(Duration::from_millis(100));
                }

                listen_decode_vin(&window_arc);
                listen_send_ports(&window_arc);
                listen_track_custom_pid(&window_arc);
                listen_connect_elm(&window_arc);

                sleep(Duration::from_secs(1));

                // load vpic database
                let _ = try_load_vpic_database(&window_arc, db_path, xz_path);

                // ensure code-descriptions and model-pids are in app_data_dir
                let _ = window_arc.emit("display-notification", FrontendNotification {
                    title: "Bases de Datos de Diagnóstico",
                    description: "Cargando bases de datos de códigos de falla y PIDs específicos del vehículo por primera vez."
                });

                let code_desc_dest = app_data_dir.join("code-descriptions.sqlite");
                if !code_desc_dest.exists() {
                    match std::fs::copy(&code_desc_src, &code_desc_dest) {
                        Ok(_) => println!("[LOAD_DIAG_DBS] Copied `code-descriptions.sqlite` to app data dir."),
                        Err(err) => {
                            println!("[LOAD_DIAG_DBS] Failed to copy `code-descriptions.sqlite`: {err}");
                            let _ = window_arc.emit("display-notification", FrontendNotification {
                                title: "Bases de Datos de Diagnóstico",
                                description: "Error al cargar la base de datos de códigos de falla. La descripción de DTCs no estará disponible."
                            });
                        }
                    }
                } else {
                    println!("[LOAD_DIAG_DBS] `code-descriptions.sqlite` already in app data dir.");
                }

                let mode22_pids_dest = app_data_dir.join("model-pids.sqlite");
                if !mode22_pids_dest.exists() {
                    match std::fs::copy(&mode22_pids_src, &mode22_pids_dest) {
                        Ok(_) => println!("[LOAD_DIAG_DBS] Copied `model-pids.sqlite` to app data dir."),
                        Err(err) => {
                            println!("[LOAD_DIAG_DBS] Failed to copy `model-pids.sqlite`: {err}");
                            let _ = window_arc.emit("display-notification", FrontendNotification {
                                title: "Bases de Datos de Diagnóstico",
                                description: "Error al cargar la base de datos de PIDs Mode $22. Los PIDs específicos del vehículo no estarán disponibles."
                            });
                        }
                    }
                } else {
                    println!("[LOAD_DIAG_DBS] `model-pids.sqlite` already in app data dir.");
                }

                let _ = window_arc.emit("display-notification", FrontendNotification {
                    title: "Bases de Datos de Diagnóstico",
                    description: "Bases de datos de diagnóstico cargadas correctamente."
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
