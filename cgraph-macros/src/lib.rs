//! 
//! MDMT | Calculation-Graph macros
//! 
//! Макрос для автоматической генерации зависимостей исходных данных и вычислений
use syn::visit_mut::VisitMut;
use syn::{ExprMethodCall, Local, Expr, Ident, Error, spanned::Spanned};
use syn::visit::Visit; // Для первого прохода достаточно обычного Visit

///
/// Подготовка и структура визитора
struct EvalVisitor {
    // Имя переменной контекста (найдем на шаге 1)
    pub ctx_ident: Option<Ident>,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
    pub errors: Vec<Error>,
}

impl EvalVisitor {
    fn new() -> Self {
        Self {
            ctx_ident: None,
            reads: Vec::new(),
            writes: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// Поиск переменной контекста
/// Мы можем сделать предварительный проход по AST, чтобы просто найти первое типизированное обращение.
struct CtxFinder {
    pub ctx_ident: Option<Ident>,
}

impl<'ast> Visit<'ast> for CtxFinder {
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

/// Основной проход (Сбор зависимостей и контроль)
/// Теперь реализуем VisitMut для EvalVisitor. Он будет проверять правила, собирать типы и вычищать наши кастомные атрибуты.
impl VisitMut for EvalVisitor {
    // 1. Отлов передачи контекста в другие функции (Indirection)
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        // Проверяем наличие атрибута #[eval_dependency(Skip)]
        let mut skip_indirection = false;
        
        if let Some(attrs) = get_mut_attrs(node) {
            let original_len = attrs.len();
            attrs.retain(|attr| {
                if attr.path.is_ident("eval_dependency") { // Упрощенная проверка
                    skip_indirection = true;
                    false // Удаляем атрибут из AST
                } else {
                    true // Оставляем остальные
                }
            });
        }

        // Если это вызов функции или метода
        if let Expr::Call(call) = node {
            if !skip_indirection {
                self.check_arguments_for_ctx(&call.args, call.span());
            }
        } else if let Expr::MethodCall(method_call) = node {
            if !skip_indirection {
                self.check_arguments_for_ctx(&method_call.args, method_call.span());
            }
        }

        syn::visit_mut::visit_expr_mut(self, node);
    }

    // 2. Отлов обращений к самому контексту
    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        let method = node.method.to_string();
        let is_ctx_call = /* проверка, что receiver == self.ctx_ident */;

        if is_ctx_call && (method == "read" || method == "read_ref" || method == "write") {
            // Проверка Turbofish
            if let Some(turbofish) = &node.turbofish {
                // Извлекаем тип и кладем в self.reads или self.writes
                let type_name = extract_type_name(turbofish);
                if method == "write" {
                    self.writes.push(type_name);
                } else {
                    self.reads.push(type_name);
                }
            } else {
                // Если Turbofish нет, проверяем, не находимся ли мы внутри типизированного let.
                // (Для этого потребуется чуть расширить визитор, чтобы он запоминал состояние внутри visit_local_mut)
                // Если типа нет вообще -> генерируем ошибку:
                self.errors.push(Error::new(
                    node.span(),
                    format!("Неявное обращение к контексту. Укажите тип явно: ctx.{}::<Type>()", method)
                ));
            }
        }
        syn::visit_mut::visit_expr_method_call_mut(self, node);
    }
}
