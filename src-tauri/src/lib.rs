// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_probability_distribution(flips: u32) -> Vec<f64> {
    let mut distribution = vec![0.0; (flips + 1) as usize];
    let total_combinations = 1 << flips;

    for i in 0..total_combinations {
        let heads = (0..flips).filter(|j| (i & (1 << j)) != 0).count();
        distribution[heads] += 1.0 / total_combinations as f64;
    }

    distribution
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_probability_distribution])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
