//! 
//! MDMT | Calculation-Graph macros
//! 
//! Макрос для автоматической генерации зависимостей исходных данных и вычислений
use proc_macro2::TokenStream;
// use syn::token::Type;
use syn::{ExprCall, ItemImpl, parse_macro_input, visit_mut};
use syn::visit_mut::VisitMut;
use syn::{ExprMethodCall, Expr, Ident, Error, spanned::Spanned};
use syn::visit::Visit; // Для первого прохода достаточно обычного Visit
use quote::quote;

///
/// Подготовка и структура визитора
struct EvalVisitor {
    /// Имя переменной контекста (найдем на шаге 1)
    pub ctx_ident: Ident,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
    pub errors: Vec<Error>,
}
//
impl EvalVisitor {
    ///
    /// Returns EvalVisitor
    fn new(ctx_ident: Ident) -> Self {
        Self {
            ctx_ident,
            reads: Vec::new(),
            writes: Vec::new(),
            errors: Vec::new(),
        }
    }
    ///
    /// Вспомогательный метод для очистки атрибута #[eval_dependency(Skip)]
    fn filter_skip_attr(attrs: &mut Vec<syn::Attribute>) -> bool {
        let mut has_skip = false;
        attrs.retain(|attr| {
            if attr.path().is_ident("eval_dependency") {
                // В реальном коде тут можно глубже проверить токен `Skip`
                has_skip = true;
                false // Удаляем атрибут, так как компилятор Rust отклонит код с неизвестным атрибутом [cite: 372]
            } else {
                true
            }
        });
        has_skip
    }
}

/// Поиск переменной контекста
/// Мы можем сделать предварительный проход по AST, чтобы просто найти первое типизированное обращение.
struct CtxFinder {
    pub ctx_ident: Option<Ident>,
}
//
impl<'ast> Visit<'ast> for CtxFinder {
    ///
    /// Поиск переменной контекста
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method = node.method.to_string();
        if method == "read" || method == "read_ref" || method == "write" {
            // Если у метода есть turbofish (::<Type>)
            if node.turbofish.is_some() {
                if let Expr::Path(path) = &*node.receiver {
                    if let Some(ident) = path.path.get_ident() {
                        self.ctx_ident = Some(ident.clone());
                        return; // Нашли, дальше не ищем
                    }
                }
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}
///
/// Основной проход (Сбор зависимостей и контроль)
/// Теперь реализуем VisitMut для EvalVisitor. Он будет проверять правила, собирать типы и вычищать наши кастомные атрибуты.
impl VisitMut for EvalVisitor {
    // Перехватываем вызовы обычных функций `foo(ctx)`
    fn visit_expr_call_mut(&mut self, node: &mut ExprCall) {
        if Self::filter_skip_attr(&mut node.attrs) {
            return; // Пропускаем проверку, если есть атрибут Skip
        }
        for arg in &node.args {
            if let Expr::Path(expr_path) = arg {
                if expr_path.path.is_ident(&self.ctx_ident) {
                    self.errors.push(syn::Error::new_spanned(
                        arg,
                        "Запрещена передача контекста в другие методы. Сначала прочитайте данные, либо разбейте алгоритм на независимые классы (шаги расчета)." // [cite: 306]
                    ));
                }
            }
        }
        visit_mut::visit_expr_call_mut(self, node);
    }
    // Перехватываем вызовы методов `self.calc(ctx)` и ищем наши `read/write`
    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        if Self::filter_skip_attr(&mut node.attrs) {
            return;
        }
        // Защита от Indirection (передачи в аргументы методов)
        for arg in &node.args {
            if let Expr::Path(expr_path) = arg {
                if expr_path.path.is_ident(&self.ctx_ident) {
                    self.errors.push(syn::Error::new_spanned(
                        arg,
                        "Запрещена передача контекста в другие методы."
                    ));
                }
            }
        }
        // Ищем обращения именно к нашему контексту
        if let Expr::Path(receiver_path) = &*node.receiver {
            if receiver_path.path.is_ident(&self.ctx_ident) {
                let method_name = node.method.to_string();
                if method_name == "read" || method_name == "read_ref" || method_name == "write" {
                    // Проверяем наличие turbofish (::)
                    if let Some(syn::GenericArgument::Type(ty)) = node.turbofish.as_ref().and_then(|t| t.args.first()) {
                        let type_name = quote!(#ty).to_string();
                        if method_name == "write" {
                            if !self.writes.contains(&type_name) {
                                self.writes.push(type_name);
                            }
                        } else {
                            if !self.reads.contains(&type_name) {
                                self.reads.push(type_name);
                            }
                        }
                    } else {
                        self.errors.push(syn::Error::new_spanned(
                            &node.method,
                            "Обязательно используйте turbofish: ctx.read::<Type>()." // [cite: 287]
                        ));
                    }
                }
            }
        }
        visit_mut::visit_expr_method_call_mut(self, node);
    }
    // Обрабатываем Local (let x = ...), чтобы вырезать атрибуты и там, если нужно
    fn visit_local_mut(&mut self, node: &mut syn::Local) {
        Self::filter_skip_attr(&mut node.attrs);
        visit_mut::visit_local_mut(self, node);
    }
}
//
#[proc_macro_attribute]
pub fn eval_step(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut impl_block = parse_macro_input!(item as ItemImpl);
    // Ищем переменную
    let mut finder = CtxFinder { ctx_ident: None };
    finder.visit_item_impl(&impl_block);
    let ctx_ident = match finder.ctx_ident {
        Some(ident) => ident,
        None => {
            return Error::new(impl_block.span(), "Не найдено ни одного типизированного обращения к контексту")
                .to_compile_error()
                .into();
        }
    };
    // Валидируем и собираем граф
    let mut visitor = EvalVisitor::new(ctx_ident);
    visitor.visit_item_impl_mut(&mut impl_block); // Мутируем AST (удаляем Skip)
    if !visitor.errors.is_empty() {
        let mut compile_errors = proc_macro2::TokenStream::new();
        for err in visitor.errors {
            compile_errors.extend(err.to_compile_error());
        }
        return compile_errors.into();
    }
    // Генерируем код трейта StepTags на основе visitor.reads и visitor.writes
    let generated_tags = generate_step_tags(&impl_block, visitor);
    // Возвращаем оригинальный (но очищенный от Skip) код + новый трейт
    let expanded = quote! {
        #impl_block
        #generated_tags
    };

    proc_macro::TokenStream::from(expanded)
}
///
/// Генератор реализации трейта EvalTags
fn generate_step_tags(
    target_struct: &ItemImpl,
    visitor: EvalVisitor,
) -> TokenStream {
    // Если были архитектурные ошибки, компилируем их в TokenStream
    if !visitor.errors.is_empty() {
        let compile_errors = visitor.errors.iter().map(|e| e.to_compile_error());
        return quote! {
            #(#compile_errors)*
        };
    }
    // Формируем литералы для массивов
    let read_tags = visitor.reads.iter().map(|s| quote! { #s });
    let write_tags = visitor.writes.iter().map(|s| quote! { #s });
    quote! {
        impl EvalTags for #target_struct {
            fn tags(&self) -> CalculationDependencies {
                CalculationDependencies {
                    read: vec![#(#read_tags),*],
                    write: vec![#(#write_tags),*],
                }
            }
        }
    }
}