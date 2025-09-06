// 1. cmd action 不能用返回值。
// 2. 只有最后一个参数可以是 数组这样的集合类型。
// 3. can命令的名称不能重复。

use syn::spanned::Spanned;

use crate::FnInfo;

pub fn rule_check(attr: proc_macro::TokenStream, input: &syn::ItemFn) -> syn::Result<()> {
    rule_1(input)?;

    rule_2(attr.clone(), input)?;

    rule_3(attr, input)?;

    return Ok(());
}

/// rule 1 被 #[command] 标记的函数不能有返回值。
pub fn rule_1(input: &syn::ItemFn) -> syn::Result<()> {
    // 检查返回值
    if let syn::ReturnType::Type(_, ty) = &input.sig.output {
        // 有返回值 → 报错

        return Err(syn::Error::new_spanned(
            ty,
            "被 #[command] 标记的函数不能有返回值。",
        ));
    }

    return Ok(());
}

/// rule 2. 只有最后一个参数可以是 数组这样的集合类型。
pub fn rule_2(attr: proc_macro::TokenStream, input: &syn::ItemFn) -> syn::Result<()> {
    let mut fn_info = crate::FnInfo::new(attr, input)?;

    // 1. 去掉最后一个参数。
    let _poped = fn_info.func_args.pop();

    // input.sig.inputs
    let mut index: usize = 0;

    // 2. 检查开头到倒数第二个参数是否有 Vec 这样的集合类型， 如果有就报错。
    for a in fn_info.func_args {
        if a.is_colection_type() {
            let arg = input
                .sig
                .inputs
                .get(index)
                .expect(concat!(file!(), line!()));

            let ty_span = get_ty_span(arg);
            return Err(syn::Error::new(
                ty_span,
                "只有最后一个参数可以使用 Vec<T: FromArgStr> 类型。",
            ));
        }

        index += 1;
    }

    // evry thing's ok.
    return Ok(());
}

#[allow(dead_code)]
/// rule 3. 命令的名称不能重复。
pub fn rule_3(attr: proc_macro::TokenStream, fn_item: &syn::ItemFn) -> syn::Result<()> {
    let _new = FnInfo::new(attr, fn_item)?;

    let key = _new.gen_key();
    let re = crate::data::COMMANDS.lock();

    match re {
        Err(e) => Err::<(), syn::Error>(syn::Error::new(fn_item.span(), e.to_string())),
        Ok(mut m) => {
            // let _old_cmd = m.insert(info.func_name.clone(), info.clone());
            let _ = m.insert(key, _new.clone());
            let same_name = m
                .iter()
                .find(|f| f.1.func_name == _new.func_name && f.1.gen_key() != _new.gen_key());

            match same_name {
                None => Ok(()),

                Some(old) => {
                    let old_name = old.1.func_name.clone();
                    let old_path =
                        format!("{}:{}:{}", old.1.local_file_path, old.1.line, old.1.colum);

                    let new_name = _new.func_name;
                    let new_path = format!("{}:{}:{}", _new.local_file_path, _new.line, _new.colum);

                    let msg = format!(
                        "命令的名称不能相同：
{old_name} at: {old_path}
{new_name} at: {new_path}
                        "
                    );

                    let err = syn::Error::new(fn_item.span(), msg);
                    return Err(err);
                }
            }
        }
    }
}

/// tools functions。
fn get_ty_span(arg: &syn::FnArg) -> proc_macro2::Span {
    use syn::spanned::Spanned;
    match arg {
        syn::FnArg::Receiver(_) => arg.span().clone(),
        syn::FnArg::Typed(pat_type) => {
            let ty = pat_type.ty.as_ref();

            match ty {
                syn::Type::Array(type_array) => type_array.span(),
                syn::Type::BareFn(type_bare_fn) => type_bare_fn.span(),
                syn::Type::Group(type_group) => type_group.span(),
                syn::Type::ImplTrait(type_impl_trait) => type_impl_trait.span(),
                syn::Type::Infer(type_infer) => type_infer.span(),
                syn::Type::Macro(type_macro) => type_macro.span(),
                syn::Type::Never(type_never) => type_never.span(),
                syn::Type::Paren(type_paren) => type_paren.span(),
                syn::Type::Path(type_path) => type_path.span(),
                syn::Type::Ptr(type_ptr) => type_ptr.span(),
                syn::Type::Reference(type_reference) => type_reference.span(),
                syn::Type::Slice(type_slice) => type_slice.span(),
                syn::Type::TraitObject(type_trait_object) => type_trait_object.span(),
                syn::Type::Tuple(type_tuple) => type_tuple.span(),
                syn::Type::Verbatim(token_stream) => token_stream.span(),
                _ => pat_type.span(),
            }
        }
    }
}
