
pub fn validate_week_days(weekdays: &Vec<String>) -> Result<(), String> {
    let allowed_week_days = vec![
        "mon".to_string(),
        "tue".to_string(),
        "wed".to_string(),
        "thu".to_string(),
        "fri".to_string(),
        "sat".to_string(),
        "sun".to_string()
    ];

    for day in weekdays {
        if !allowed_week_days.contains(&day.to_lowercase()) {
            return Err(format!("{} is not a valid day", day))
        }
    }

    Ok(())
}
