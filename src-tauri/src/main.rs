
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::sync::Mutex;
pub(crate) mod keyence;
use tauri::{Manager, State};
pub struct Host (Mutex<String>);
pub struct Port(Mutex<String>);
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn change_host_port(host_state: State<Host>, port_state: State<Port>, host:String, port:String) -> Result<String,String>{
    let mut host_state = host_state.0.lock().unwrap();
    *host_state = host.to_string();
    let mut port_state = port_state.0.lock().unwrap();
    *port_state = port.to_string();
    Ok(format!("設定完了->{}：{}",&host_state,&port_state))
}
#[tauri::command]
fn read_plc(host_state: State<Host>, port_state: State<Port>,address:String) -> Result<String,String>{
    let host = host_state.0.lock().unwrap();
    let port = port_state.0.lock().unwrap();
    let msg = keyence::read(&*host.to_string(),&*port.to_string(), &address);
    msg
}
#[tauri::command]
fn write_plc(host_state: State<Host>, port_state: State<Port>,address:String,data:String) -> Result<String,String>{
    let host = host_state.0.lock().unwrap();
    let port = port_state.0.lock().unwrap();
    println!("send data");
    let msg = keyence::write(&*host.to_string(),&*port.to_string(), &address,&data);
    msg
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            change_host_port,
            read_plc,
            write_plc
        ])
        .setup(|app| {
            let host = Mutex::new("".to_string());
            let port = Mutex::new("".to_string());
            app.manage(Host(host));
            app.manage(Port(port));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
