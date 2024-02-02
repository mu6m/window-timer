use active_win_pos_rs::get_active_window;
use std::{thread, time};
use std::collections::HashMap;

#[derive(Clone,Debug)]
struct WindowInfo {
    name: String,
    title: Vec<String>,
    time: u64,
}

fn main() {
    let seconds = 1 * 1;
    let sleep_time = time::Duration::from_millis(1000 * seconds);
    let mut watch_time: HashMap<String, WindowInfo> = HashMap::new();
    loop {
        match get_active_window() {
            Ok(active_window) => {
                if watch_time.contains_key(&active_window.process_name){
                    let mut value = watch_time[&active_window.process_name].clone();
                    if !value.title.contains(&active_window.process_name){
                        value.title.push(active_window.process_name.clone())
                    }
                    value.time = value.time + seconds;
                    watch_time.insert(
                        active_window.process_name.clone(),
                        value
                    );
                }else{
                    watch_time.insert(
                        active_window.process_name.clone(),
                        WindowInfo {
                            name: active_window.process_name.clone(),
                            title: vec![active_window.title.clone()],
                            time: 1,
                        },
                    );
                }
                println!("{:#?}", watch_time);
                println!("{:#?}", active_window);
            },
            Err(()) => {
                println!("error occurred while getting the active window");
            }
        }

        thread::sleep(sleep_time);
    }
}