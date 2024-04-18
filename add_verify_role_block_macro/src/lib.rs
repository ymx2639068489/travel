extern crate proc_macro;
use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn verify_permissions(attr: TokenStream, input: TokenStream) -> TokenStream {
  let arg = attr.to_string();
  let (_, mut rule_value) = arg.split_at(arg.find(",").unwrap() + 1);
  rule_value = rule_value.trim();
  let (table, _) = arg.split_at(arg.find(",").unwrap());
  let res = build_code(input, table, rule_value);
  res
}

fn build_code(input: TokenStream, table: &str, rule_value: &str) -> TokenStream {

  // 解析输入的函数
  let mut func = parse_macro_input!(input as ItemFn);

  // 构造函数体的代码
  let func_block = &func.block;
  let output = quote! {
    {
      let __log_result = {
        let __res = jwt.validate_role(&pool, #table, #rule_value).await;
        // 验证通过了继续，否则退出
        if !__res {
          Ok(Response::client_error("权限不够"))
        } else {
          #func_block
        }
      };
      __log_result
    }
  };

  // 将函数体替换为新的代码
  func.block = syn::parse2(output).unwrap();

  // 将新的函数定义转换回 TokenStream
  (quote! { #func }).into()
}
