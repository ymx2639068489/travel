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
  
  build_code(item, table, rule_value)
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
