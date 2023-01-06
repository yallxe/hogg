use notify_rust::Notification;

pub fn show_detections_notification(domain: &String) {
    Notification::new()
        .summary("Hogg")
        .body(
            format!(
                "New vulnerability detected @ {}\nCheck CLI for more information",
                domain
            )
            .as_str(),
        )
        .show()
        .unwrap();
}
