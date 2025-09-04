// 1. cmd action 不能用返回值。
// 2. 只有最后一个参数可以是 数组这样的集合类型。
// 3. can命令的名称不能重复。

use crate::FnInfo;

pub fn rule_check(attr: proc_macro::TokenStream, input: &syn::ItemFn) -> syn::Result<()> {
    rule_1(input)?;
    // if let Some(err) = rule_1(input) {
    //     return Some(err);
    // }
    rule_2(attr, input)?;
    // if let Some(err) = rule_2(attr, input) {
    //     return Some(err);
    // }

    return Ok(());
}

/// rule 1 被 #[command] 标记的函数不能有返回值。
pub fn rule_1(input: &syn::ItemFn) -> syn::Result<()> {
    // 检查返回值
    if let syn::ReturnType::Type(_, ty) = &input.sig.output {
        // 有返回值 → 报错

        return Err(syn::Error::new_spanned(ty, "command 函数不能有返回值。"));
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
                "只有最后一个参数可以使用 Vec<T> 类型。",
            ));
        }

        index += 1;
    }

    // evry thing's ok.
    return Ok(());
}

#[allow(dead_code)]
/// rule 3. 命令的名称不能重复。
pub fn rule_3(old: Option<FnInfo>, _new: FnInfo, span: proc_macro2::Span) -> syn::Result<()> {
    if let Some(_) = old {
        let msg = format!("命令的名称不能重复");
        return Err(syn::Error::new(span, msg));
    }
    // let msg = format!("命令的名称不能重复");
    // return syn::Error::new(span, msg);

    Ok(())
}

// toos functions。

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
