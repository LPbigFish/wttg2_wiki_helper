use clipboard::ClipboardProvider;

fn main() {
    let reg = regex::Regex::new(r"(?mi)(?:https?://)(?:\([-A-Z0-9+&@#/%=~_|$?!:,.]*\)|[-A-Z0-9+&@#/%=~_|$?!:,.])*(?:\([-A-Z0-9+&@#/%=~_|$?!:,.]*\)|ann)")
        .expect("Error creating regex");

    let mut clip_instance = clipboard::ClipboardContext::new().expect("Error creating clipboard instance");

    let mut clip_content = clip_instance.get_contents().expect("Error getting clipboard content");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let new_clip_content = clip_instance.get_contents().expect("Error getting clipboard content");

        if new_clip_content != clip_content {
            if let Some(matches) = reg.captures(new_clip_content.as_str()) {
                for mat in matches.iter() {
                    if let Some(mat) = mat {
                        println!("Match: {}", mat.as_str());
                        
                        clip_instance.set_contents(mat.as_str().to_string()).expect("Error setting clipboard content");
                        clip_content = mat.as_str().to_string();
                    }
                }
            }
        }
    }
}