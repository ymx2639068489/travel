use chrono::Datelike;
use chrono::Timelike;

/**
 * 隐藏身份证中间的详细信息
 * 数据脱敏
 */
pub fn hide_id_number(id_number: String) -> String {
  let len = id_number.len();
  let mut id = id_number.clone();
  id.truncate(6);
  let (_, end) = id_number.split_at(len - 4);
  id + "********" + end
}


/**
 * 获取当前时间，并以NaiveDateTim格式返回
 */
pub fn now_to_naive_date_time() -> chrono::NaiveDateTime {
  let now = chrono::Local::now();
  let d = chrono::NaiveDate::from_ymd_opt(
    now.year(),
    now.month(),
    now.day(),
  ).unwrap();
  let t = chrono::NaiveTime::from_hms_milli_opt(
    now.hour(),
    now.minute(),
    now.second(),
    0
  ).unwrap();
  chrono::NaiveDateTime::new(d,t)
}
/**
 * 字符串转换为chrono::NaiveDateTime
 * 请使用以下格式传入参数:
 *  "%Y-%m-%d %H:%M:%S" 
 *  "2015-07-01 08:59:60"
 */
pub fn str_to_naive_date_time(str: &str) -> chrono::NaiveDateTime {
  // println!("{}", str);
  let res = chrono::NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S");

  match res {
    Ok(res) => res,
    Err(e) => {
      eprintln!("{}", e);
      now_to_naive_date_time()
    }
  }
}