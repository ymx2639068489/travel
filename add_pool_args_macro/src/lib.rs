extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn add_pool_args(_: TokenStream, item: TokenStream) -> TokenStream {
  let res = build_code(item);
  // println!("add pool args {:?}", res.to_string());
  // println!("");
  // println!("");
  res
}

fn build_code(input: TokenStream) -> TokenStream {

  let mut func = input.to_string();
  // 第一次以大括号分割，第一个大括号一般都是函数体与函数签名分割的地方
  if let Some((head, body)) = func.split_once("{") {
    // 第二次以fn 分割，因为函数签名上面可能还有一些其他的宏
    if let Some((hhead, hbody)) = head.split_once("fn") {
      let l = hbody.find("(").unwrap();
      let r = hbody.rfind(")").unwrap();

      let args = &hbody[l+1..r];

      let new_args = if args.len() == 0 {
        "pool: actix_web::web::Data<crate::DbPool>".to_owned()
      } else {
        args.to_owned() + ", pool: actix_web::web::Data<crate::DbPool>"
      };
      let new_head = "fn ".to_owned() + &hbody[0..l+1] + &new_args + &hbody[r..];
      func = hhead.to_owned() + &new_head + "{" + body;
    }
  }
  func.parse().unwrap()
}
