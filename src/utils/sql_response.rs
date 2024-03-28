
pub fn diesel_to_res(res: Result<usize, diesel::result::Error>) -> Result<bool, diesel::result::Error> {
  match res {
    Ok(size) => Ok(size != 0),
    Err(e) => Err(e)
  }
}