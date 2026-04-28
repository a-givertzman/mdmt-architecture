//! 
//! MDMT | Calculation-Graph macros
//! 
//! Макрос для автоматической генерации зависимостей исходных данных и вычислений
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, visit::Visit, visit_mut::VisitMut,
    Error, Expr, ExprCall, ExprMethodCall, ItemImpl
};

///
/// Поиск переменной контекста
#[derive(Default)]
struct CtxFinder {
    ctx_ident: Option<String>,
}
impl<'ast> Visit<'ast> for CtxFinder {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        if self.ctx_ident.is_some() { return; }
        let method = node.method.to_string();
        // Убрано требование turbofish, чтобы находить `ctx` и в let-выражениях
        if method == "read" || method == "read_ref" || method == "write" {
            if let Expr::Path(expr_path) = &*node.receiver {
                if let Some(ident) = expr_path.path.get_ident() {
                    self.ctx_ident = Some(ident.to_string());
                    return;
                }
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if self.ctx_ident.is_some() { return; }
        if let Expr::Path(expr_path) = &*node.func {
            let path_str = quote!(#expr_path).to_string().replace(" ", "");
            if path_str.contains("ContextRead") || path_str.contains("ContextWrite") {
                if let Some(arg) = node.args.first() {
                    let mut target_expr = arg;
                    if let Expr::Reference(expr_ref) = arg {
                        target_expr = &*expr_ref.expr;
                    }
                    if let Expr::Path(arg_path) = target_expr {
                        if let Some(ident) = arg_path.path.get_ident() {
                            self.ctx_ident = Some(ident.to_string());
                            return;
                        }
                    }
                }
            }
        }
        syn::visit::visit_expr_call(self, node);
    }
}

///
/// Подготовка и структура визитора
struct EvalVisitor {
    ctx_ident: String,
    reads: Vec<String>,
    writes: Vec<String>,
    errors: Vec<Error>,
}

impl EvalVisitor {
    fn new(ctx_ident: String) -> Self {
        Self {
            ctx_ident,
            reads: Vec::new(),
            writes: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Универсальный метод очистки типов от ссылок и пробелов
    fn clean_type_str(tokens: impl ToTokens) -> String {
        quote!(#tokens).to_string().replace(" ", "").replace("&", "").replace("mut", "")
    }
}

impl VisitMut for EvalVisitor {
    /// Перехватываем типизированные объявления: let initial: &InitialCtx = ctx.read();
    fn visit_local_mut(&mut self, node: &mut syn::Local) {
        if let syn::Pat::Type(pat_type) = &node.pat {
            if let Some(init) = &mut node.init {
                let mut is_ctx_call = false;
                let mut method_name = String::new();

                if let Expr::MethodCall(method_call) = &*init.expr {
                    if let Expr::Path(expr_path) = &*method_call.receiver {
                        if expr_path.path.is_ident(&self.ctx_ident) {
                            let method = method_call.method.to_string();
                            if method == "read" || method == "read_ref" || method == "write" {
                                is_ctx_call = true;
                                method_name = method;
                            }
                        }
                    }
                } else if let Expr::Call(expr_call) = &*init.expr {
                    if let Expr::Path(expr_path) = &*expr_call.func {
                        let path_str = quote!(#expr_path).to_string().replace(" ", "");
                        if path_str.contains("ContextRead") || path_str.contains("ContextWrite") {
                            if let Some(arg) = expr_call.args.first() {
                                let mut target_expr = arg;
                                if let Expr::Reference(expr_ref) = arg {
                                    target_expr = &*expr_ref.expr;
                                }
                                if let Expr::Path(arg_path) = target_expr {
                                    if arg_path.path.is_ident(&self.ctx_ident) {
                                        is_ctx_call = true;
                                        method_name = if path_str.contains("Write") {
                                            "write".to_string()
                                        } else {
                                            "read".to_string()
                                        };
                                    }
                                }
                            }
                        }
                    }
                }

                if is_ctx_call {
                    let clean_type = Self::clean_type_str(&pat_type.ty);
                    if method_name == "write" {
                        self.writes.push(clean_type);
                    } else {
                        self.reads.push(clean_type);
                    }
                    // Прерываем обход, чтобы правая часть не выдала ошибку "Нетипизированное обращение"
                    return; 
                }
            }
        }
        syn::visit_mut::visit_local_mut(self, node);
    }

    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        if let Expr::Path(expr_path) = &*node.receiver {
            if expr_path.path.is_ident(&self.ctx_ident) {
                let method = node.method.to_string();
                if method == "read" || method == "read_ref" || method == "write" {
                    if let Some(turbofish) = &node.turbofish {
                        if let Some(arg) = turbofish.args.first() {
                            let clean_type = Self::clean_type_str(arg);
                            if method == "write" {
                                self.writes.push(clean_type);
                            } else {
                                self.reads.push(clean_type);
                            }
                        }
                    } else {
                        self.errors.push(Error::new(
                            node.span(),
                            "Нетипизированное обращение к контексту. Укажите тип (например: let x: Type = ctx.read() или ctx.read::<Type>())",
                        ));
                    }
                }
            }
        }
        syn::visit_mut::visit_expr_method_call_mut(self, node);
    }

    fn visit_expr_call_mut(&mut self, node: &mut ExprCall) {
        if let Expr::Path(expr_path) = &*node.func {
            let path_str = quote!(#expr_path).to_string().replace(" ", "");
            let is_read = path_str.contains("ContextRead");
            let is_write = path_str.contains("ContextWrite");
            
            if is_read || is_write {
                let mut involves_ctx = false;
                if let Some(arg) = node.args.first() {
                    let mut target_expr = arg;
                    if let Expr::Reference(expr_ref) = arg {
                        target_expr = &*expr_ref.expr;
                    }
                    if let Expr::Path(arg_path) = target_expr {
                        if arg_path.path.is_ident(&self.ctx_ident) {
                            involves_ctx = true;
                        }
                    }
                }
                
                if involves_ctx {
                    let mut clean_type = None;
                    for segment in &expr_path.path.segments {
                        let ident_str = segment.ident.to_string();
                        if ident_str.contains("ContextRead") || ident_str.contains("ContextWrite") {
                            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(arg) = args.args.first() {
                                    clean_type = Some(Self::clean_type_str(arg));
                                    break;
                                }
                            }
                        }
                    }
                    
                    if let Some(ty) = clean_type {
                        if is_write {
                            self.writes.push(ty);
                        } else {
                            self.reads.push(ty);
                        }
                    } else {
                        self.errors.push(Error::new(
                            node.span(),
                            "Нетипизированное обращение. Укажите тип: ContextReadRef::<Type>::read(&ctx)",
                        ));
                    }
                }
            }
        }
        syn::visit_mut::visit_expr_call_mut(self, node);
    }
}

///
/// Основной проход (Сбор зависимостей и контроль)
#[proc_macro_attribute]
pub fn eval_depend(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let self_ty = ast.self_ty.clone();
    
    let mut finder = CtxFinder::default();
    finder.visit_item_impl(&ast);
    let ctx_ident = match finder.ctx_ident {
        Some(ctx_ident) => ctx_ident,
        None => {
            let error = Error::new(ast.span(), "Ни одного обращения к контексту").to_compile_error();
            return quote! {
                #error
                #ast
            }.into();
        }
    };
    
    let mut visitor = EvalVisitor::new(ctx_ident);
    visitor.visit_item_impl_mut(&mut ast);
    
    if !visitor.errors.is_empty() {
        let compile_errors = visitor.errors.iter().map(Error::to_compile_error);
        return quote! {
            #(#compile_errors)*
            #ast
        }.into();
    }
    
    visitor.reads.sort();
    visitor.reads.dedup();
    visitor.writes.sort();
    visitor.writes.dedup();
    
    let reads = visitor.reads;
    let writes = visitor.writes;
    
    let expanded = quote! {
        #ast
        impl crate::domain::EvalTags for #self_ty {
            fn tags(&self) -> crate::domain::CalculationTags {
                crate::domain::CalculationTags {
                    // quote! автоматически конвертирует String в &'static str литералы
                    read: vec![#(#reads),*],
                    write: vec![#(#writes),*],
                }
            }
        }
    };
    TokenStream::from(expanded)
}
