use chrono::Local;

pub fn get_now() -> String {
    let fmt = "%Y%m%d%H%M%S";
    let now = Local::now().format(fmt);

    return format!("{now}");
}
