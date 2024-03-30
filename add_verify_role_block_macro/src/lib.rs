extern crate proc_macro;
use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn verify_permissions(attr: TokenStream, item: TokenStream) -> TokenStream {
  let arg = attr.to_string();
  let (_, rule_value) = arg.split_at(
    arg.find(",").unwrap() + 2
  );
  let (table, _) = arg.split_at(
    arg.find(",").unwrap()
  );
  let input = add_jwt_args(item);

  // println!("add jwt args {:?}", input.to_string());
  // println!("");
  // println!("");

  let res = build_code(input, table, rule_value);

  
  // println!("add verify block {:?}", res.to_string());
  // println!("");
  // println!("");

  res
}

fn build_code(input: TokenStream, table: &str, rule_value: &str) -> TokenStream {

  // 解析输入的函数
  let mut func = parse_macro_input!(input as ItemFn);

  // 获取函数的名称
  let func_name = &func.sig.ident;
  // 构造函数体的代码
  let func_block = &func.block;
  let output = quote! {
    {
      println!("fun {} starts", stringify!(#func_name));
      let start_time = std::time::Instant::now();
      let __log_result = {
        let __res = jwt_admin_data.validate_role(#table, #rule_value);
        // 验证通过了继续，否则退出
        if !__res {
          Ok(Response::client_error("权限不够"))
        } else {
          #func_block
        }
      };
      let end_time = std::time::Instant::now();
      let duration = end_time - start_time;
      println!("funciton {} execution time: {:?}", stringify!(#func_name), duration);
      __log_result
    }
  };

  // 将函数体替换为新的代码
  func.block = syn::parse2(output).unwrap();

  // 将新的函数定义转换回 TokenStream
  quote! { #func }.into()
}

fn add_jwt_args(input: TokenStream) -> TokenStream {
  let mut func = input.to_string();
  // 第一次以大括号分割，第一个大括号一般都是函数体与函数签名分割的地方
  if let Some((head, body)) = func.split_once("{") {
    // 第二次以fn 分割，因为函数签名上面可能还有一些其他的宏
    if let Some((hhead, hbody)) = head.split_once("fn") {
      let l = hbody.find("(").unwrap();
      let r = hbody.rfind(")").unwrap();

      let args = &hbody[l+1..r];

      let new_args = if args.len() == 0 {
        "jwt_admin_data: JwtAdminData".to_owned()
      } else {
        args.to_owned() + ", jwt_admin_data: JwtAdminData"
      };
      let new_head = "fn ".to_owned() + &hbody[0..l+1] + &new_args + &hbody[r..];
      func = hhead.to_owned() + &new_head + "{" + body;
    }
  }
  func.parse().unwrap()

}