#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use clipboard_win::{formats, get_clipboard, set_clipboard};
use regex::Regex;

const REPLACEMENTS: &[(&str, &str)] = &[
    (
        r"\b(http|https)://x.com/([A-Za-z0-9_]{4, 15})/status",
        r"https://fixvx.com/$2/status",
    ),
    (
        r"\b(http|https)://twitter.com/([A-Za-z0-9_]{4, 15})/status",
        r"https://vxtwitter.com/$2/status",
    ),
    (
        r"\b(http|https)://(www\.)tiktok.com/@([A-Za-z0-9_.]{4, 15})/video",
        r"https://www.vxtiktok.com/@$3/video",
    ),
    (
        r"\b(http|https)://vm.tiktok.com/([A-Za-z0-9_.-]+)",
        r"https://vm.vxtiktok.com/$2",
    ),
    (
        r"\b(http|https)://(www[.])?pixiv.net/([a-z]{2}/)?artworks/([0-9]+)",
        r"https://www.pixiv.net/en/artworks/$4",
    ),
    (r"\b(http|https)://bsky.app/(.*)", r"https://bskyx.app/$2"),
];

fn main() {
    println!(
        "Raphii's Clipboard Automations v{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Listening for clipboard changes...");
    let result = Master::new(Handler).run();
    if let Err(e) = result {
        eprintln!("Couldn't listen for clipboard changes: {}", e)
    }
}

struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // Get clipboard content
        let content: String = match get_clipboard(formats::Unicode) {
            Ok(clipboard) => clipboard,
            Err(_) => {
                return CallbackResult::Next;
            }
        };

        // Attempt to modify content
        let modified = patch_clipboard_content(content.clone());

        // Set the clipboard if changes were made
        if modified != content {
            match set_clipboard(formats::Unicode, modified.clone()) {
                Ok(_) => {
                    println!(
                        "Clipboard content updated!\nPrevious Content: {}\nNew Content: {}",
                        content, modified
                    );
                }
                Err(e) => {
                    eprintln!("Couldn't update clipboard content: {}", e);
                }
            }
        }

        CallbackResult::Next
    }
}

fn patch_clipboard_content(content: String) -> String {
    let mut content = content;
    for (pattern, replacement) in REPLACEMENTS.iter() {
        content = Regex::new(pattern)
            .unwrap()
            .replace_all(&content, *replacement)
            .to_string();
    }
    content
}
