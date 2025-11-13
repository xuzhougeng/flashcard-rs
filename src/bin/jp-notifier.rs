use std::process::Command;
use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_YESNO, IDYES, MB_SYSTEMMODAL};

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn ask_show_card(romaji: &str) -> bool {
    let title = to_wide("JP Reminder");
    let body = to_wide(&format!(
        "Romaji: {}\n\n选择 Yes 打开卡片，No 跳过",
        romaji.to_uppercase()
    ));
    unsafe {
        let res = MessageBoxW(
            std::ptr::null_mut(),
            body.as_ptr(),
            title.as_ptr(),
            MB_YESNO | MB_ICONINFORMATION | MB_SYSTEMMODAL,
        );
        res == IDYES
    }
}

fn open_cli_card(romaji: &str) {
    // Open a new console window that runs: jp <romaji>
    // Using cmd start to ensure a separate window
    let _ = Command::new("cmd")
        .args(["/C", "start", "", "jp", romaji])
        .spawn();
}

fn main() {
    // Simple arg: --interval-minutes=N, default 30
    let mut interval_minutes: u64 = 30;
    let args: Vec<String> = std::env::args().collect();
    for w in args.windows(2) {
        if w[0] == "--interval-minutes" {
            if let Ok(v) = w[1].parse::<u64>() {
                interval_minutes = v;
            }
        }
    }

    // Basic romaji set (gojūon) to quiz from
    const ROMAJI: &[&str] = &[
        "a","i","u","e","o",
        "ka","ki","ku","ke","ko",
        "sa","shi","su","se","so",
        "ta","chi","tsu","te","to",
        "na","ni","nu","ne","no",
        "ha","hi","fu","he","ho",
        "ma","mi","mu","me","mo",
        "ya","yu","yo",
        "ra","ri","ru","re","ro",
        "wa","wo","n",
    ];

    let interval = Duration::from_secs(interval_minutes * 60);
    let mut rng = thread_rng();

    // Show immediately on start
    loop {
        let pick = ROMAJI.choose(&mut rng).unwrap();
        if ask_show_card(pick) {
            open_cli_card(pick);
        }
        // Sleep for interval
        thread::sleep(interval);
    }
}