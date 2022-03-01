use std::thread;

use regex::bytes::Regex;

use binaryninja::binaryview::{BinaryView, BinaryViewExt};
use binaryninja::command::register;
use binaryninja::interaction::get_text_line_input;
use binaryninja::logger;
use binaryninja::{
    backgroundtask::BackgroundTask, binaryview::BinaryViewBase,
};
use log::{error, info, LevelFilter};

fn generate_regex(raw: &str) -> Option<Regex> {
    let mut res = raw
        .to_string()
        .split_whitespace()
        .map(|x| match &x {
            &"?" => ".".to_string(),
            x => format!("\\x{}", x),
        })
        .collect::<Vec<_>>()
        .join("");
    res.insert_str(0, "(?s-u)");
    Regex::new(&res).ok()
}

fn scan_binary(_task: &mut BackgroundTask, view: &BinaryView) {
    if let Some(regex) =
        get_text_line_input("Byte search pattern", "BFF - Byte search pattern")
    {
        let binary = view.read_vec(0, view.len());
        if let Some(reg) = generate_regex(regex.as_str()) {
            for m in reg.find_iter(&binary[..]) {
                info!(
                    "BFF - Byte match found - 0x{:x} --> 0x{:x}: {}",
                    m.start(),
                    m.end(),
                    regex.as_str()
                );
            }
        } else {
            error!("BFF - No byte matches found - {}", regex.as_str());
        }
    }
}

fn scan_binary_bg(view: &BinaryView) {
    let view_ref = view.to_owned();
    thread::spawn(move || {
        if let Ok(mut task) = BackgroundTask::new("BFF: Scanning...", true) {
            scan_binary(&mut task, &view_ref);
            task.finish();
        };
    });
}

#[no_mangle]
pub extern "C" fn UIPluginInit() -> bool {
    if logger::init(LevelFilter::Info).is_err() {
        error!("Initialization failed, skipping IEF plugin...");
        return false;
    }

    register(
        "Find byte pattern",
        "Find pattern of bytes utilizing IDA style byte pattern matching",
        scan_binary_bg,
    );

    true
}
