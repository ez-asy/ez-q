use actix_web::middleware::Logger;

pub fn create_logger_mw() -> Logger {
    Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D")
}
