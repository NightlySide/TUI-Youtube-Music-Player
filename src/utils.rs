use std::cmp::{max, min};

use google_youtube3::api::Video;

pub fn parse_time(old_time: &str) -> String {
    let mut new_time = String::new();

    let old_time = old_time.split("PT").nth(1).unwrap().split("S").nth(0).unwrap();
    let splitted: Vec<&str> = old_time.split("H").collect();
    // there is hours
    let old_time = if splitted.len() > 1 {
        new_time.push_str(&format!("{}:", splitted.get(0).unwrap()));
        splitted.get(1).unwrap()
    } else {
        splitted.get(0).unwrap()
    };

    let splitted: Vec<&str> = old_time.split("M").collect();
    // there is minutes
    let old_time = if splitted.len() > 1 {
        new_time.push_str(&format!("{}:", splitted.get(0).unwrap()));
        splitted.get(1).unwrap()
    } else {
        splitted.get(0).unwrap()
    };

    new_time.push_str(old_time);

    new_time
}

pub fn parse_quantities(number: &str) -> String {
    let nb = number.parse::<f32>().unwrap();
    
    let millnames = vec!["", "k", "M", "B"];
    let idx = max(0, min(millnames.len() - 1, if nb == 0.0 { 0 } else { (nb.abs().log10() / 3.0) as usize }));

    format!("{:.1}{}", nb / (10_i32.pow(3 * idx as u32) as f32), millnames.get(idx).unwrap())
}

pub fn display_video_line(video: Video) -> String {
    let details = video.content_details.unwrap();
    let duration = details.duration.unwrap();

    let stats = video.statistics.unwrap();
    let likes = stats.like_count.unwrap();
    let views = stats.view_count.unwrap();

    let snippet = video.snippet.unwrap();
    let title = snippet.title.unwrap();

    format!("{} ({}) - Views: {} ({} likes)",
        title,
        parse_time(&duration),
        parse_quantities(&views),
        parse_quantities(&likes),
    )
}