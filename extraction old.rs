use rem_utils::fmt_file;
use std::{
    fs::{self},
    io::{
        self,
        ErrorKind
    },
    iter,
    ops::RangeInclusive,
};
use ast::make;
use either::Either;
use hir::{
    HasSource, HirDisplay, InFile, Local, LocalSource, PathResolution, Semantics,
    TypeInfo, TypeParam,
};
use ide_db::{
    defs::{Definition, NameRefClass},
    imports::insert_use::ImportScope,
    search::{FileReference, ReferenceCategory, SearchScope},
    source_change::SourceChangeBuilder,
    syntax_helpers::node_ext::{
        for_each_tail_expr, preorder_expr, walk_expr, walk_pat, walk_patterns_in_expr,
    },
    FxIndexSet, RootDatabase,
};
use syntax::{
    ast::{
        self, edit::IndentLevel, edit_in_place::Indent, AstNode, AstToken, HasGenericParams,
    },
    match_ast, ted, Edition, SyntaxElement,
    SyntaxKind::{self, COMMENT},
    SyntaxNode, SyntaxToken, TextRange, TextSize, TokenAtOffset, WalkEvent, T,
};

use ide_assists::assist_context::{
        AssistContext,
        TreeMutator,
    };

use crate::error::ExtractionError;

#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
    pub line: usize, // Line in file, 1-indexed
    pub column: usize, // Column in line, 1-indexed
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Cursor {
        Cursor { line, column }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExtractionInput {
    pub file_path: String,
    pub output_path: String,
    pub new_fn_name: String,
    pub start_cursor: Cursor,
    pub end_cursor: Cursor,
}

impl ExtractionInput {
    pub fn new(
        file_path: &str,
        output_path: &str,
        new_fn_name: &str,
        start_cursor: Cursor,
        end_cursor: Cursor,
    ) -> ExtractionInput {
        ExtractionInput {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_cursor,
            end_cursor,
        }
    }

    pub fn new_raw(
        file_path: &str,
        output_path: &str,
        new_fn_name: &str,
        start_cursor: usize,
        start_column: usize,
        end_cursor: usize,
        end_column: usize,
    ) -> ExtractionInput {
        ExtractionInput {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_cursor: Cursor::new(start_cursor, start_column),
            end_cursor: Cursor::new(end_cursor, end_column),
        }
    }
}

// ========================================
// Checks for the validity of the input
// ========================================

// Check if the file exists and is readable
fn check_file_exists(file_path: &str) -> Result<(), ExtractionError> {
    if fs::metadata(file_path).is_err() {
        return Err(ExtractionError::Io(io::Error::new(
            ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        )));
    }
    Ok(())
}

fn check_line_numbers(input: &ExtractionInput) -> Result<(), ExtractionError> {
    // Since the cursor is 1-indexed, we need to check if the line number is 0
    if input.start_cursor.line == 0 {
        return Err(ExtractionError::ZeroLineIndex);
    }
    // Same for the end cursor
    if input.end_cursor.line == 0 {
        return Err(ExtractionError::ZeroLineIndex);
    }

    if input.start_cursor.line > input.end_cursor.line {
        return Err(ExtractionError::InvalidLineRange);
    }

    let source_code: String = fs::read_to_string(&input.file_path)?;
    let num_lines = source_code.lines().count();
    if input.end_cursor.line >= num_lines {
        return Err(ExtractionError::InvalidLineRange);
    }

    Ok(())
}

fn check_columns(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.start_cursor.line == input.end_cursor.line
        && input.start_cursor.column > input.end_cursor.column
    {
        return Err(ExtractionError::InvalidColumnRange);
    }

    Ok(())
}

fn check_cursor_not_equal(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.start_cursor == input.end_cursor {
        return Err(ExtractionError::SameCursor);
    }

    Ok(())
}

fn verify_input(input: &ExtractionInput) -> Result<(), ExtractionError> {
    // Execute each input validation step one by one
    check_file_exists(&input.file_path)?;
    check_line_numbers(input)?;
    check_columns(input)?;
    check_cursor_not_equal(input)?;

    Ok(())
}

// ========================================
// Performs the method extraction
// ========================================

// Function to extract the code segment based on cursor positions
pub fn extract_method(input: ExtractionInput) -> Result<String, ExtractionError> {
    // Get the cursor positions
    let start_cursor: Cursor = input.clone().start_cursor;
    let end_cursor: Cursor = input.clone().end_cursor;

    // Get info about the files
    let input_path: &str = &input.file_path;
    let output_path: &str = &input.output_path;
    let new_fn_name: &str = &input.new_fn_name;

    verify_input(&input)?;

    // Call out to rust-analyzer to get the context of the file
    let ctx: &AssistContext<'_> = &get_assist_context(&input_path, start_cursor, end_cursor)?;

    // Similar Logic to the `extract_function` function from rust-analyzer
    // Slight difference in that the extracted code is written to a new file

    let range = ctx.selection_trimmed();

    let node = ctx.covering_element();
    if matches!(node.kind(), T!['{'] | T!['}'] | T!['('] | T![')'] | T!['['] | T![']']) {
        cov_mark::hit!(extract_function_in_braces_is_not_applicable);
        return Err(ExtractionError::FunctionInBraces);
    }

    if node.kind() == COMMENT {
        cov_mark::hit!(extract_function_in_comment_is_not_applicable);
        return Err(ExtractionError::Comment);
    }

    let node = match node {
        syntax::NodeOrToken::Node(n) => n,
        syntax::NodeOrToken::Token(t) => t.parent().ok_or(ExtractionError::Undefined)?,
    };

    let body = extraction_target(&node, range).ok_or(ExtractionError::Undefined)?;

    let (locals_used, self_param) = body.analyze(&ctx.sema);

    let anchor = if self_param.is_some() { Anchor::Method } else { Anchor::Freestanding };
    let insert_after = node_to_insert_after(&body, anchor).ok_or(ExtractionError::Undefined)?;
    let semantics_scope = ctx.sema.scope(&insert_after).ok_or(ExtractionError::Undefined)?;
    let module = semantics_scope.module();
    let edition = semantics_scope.krate().edition(ctx.db());

    let (container_info, contains_tail_expr) = body.analyze_container(&ctx.sema, edition).ok_or(ExtractionError::Undefined)?;

    let ret_ty = body.return_ty(ctx).ok_or(ExtractionError::Undefined)?;
    let control_flow = body.external_control_flow(ctx, &container_info).ok_or(ExtractionError::Undefined)?;
    let ret_values = body.ret_values(ctx, node.parent().as_ref().unwrap_or(&node));

    let target_range = body.text_range();
    let scope = ImportScope::find_insert_use_container(&node, &ctx.sema).ok_or(ExtractionError::Undefined)?;

    let fun = Function {
        name: make_function_name(&semantics_scope, new_fn_name),
        self_param,
        params: body.extracted_function_params(ctx, &container_info, locals_used.iter().copied()),
        control_flow,
        ret_ty,
        body,
        outliving_locals: ret_values.collect(),
        contains_tail_expr,
        mods: container_info,
    };

    let new_indent = IndentLevel::from_node(&insert_after);
    let old_indent = fun.body.indent_level();

    let insert_after = builder.make_syntax_mut(insert_after);

    let call_expr = make_call(ctx, &fun, old_indent);
    


    // TODO: Write the extracted code to a new file

    // Format the output file with rustfmt
    let _ = fmt_file(output_path, &vec![]); // Just returns the command so can be ignored

    // TODO: Return the refactored code as a string
    Ok("".to_string())

}

// Calls out to rust-analyzer to get the context of the file
// Requires rust-analyzer to be installed on the system
fn get_assist_context(
    file_path: &str,
    start_cursor: Cursor, // (line, column), 1-indexed
    end_cursor: Cursor,
) -> Result<AssistContext<'static>, ExtractionError> {

    todo!()
}

fn make_function_name(semantics_scope: &hir::SemanticsScope<'_>, new_fn_name: &str) -> ast::NameRef {
    let mut names_in_scope = vec![];
    semantics_scope.process_all_names(&mut |name, _| {
        names_in_scope.push(
            name.display(
                semantics_scope.db.upcast(),
                semantics_scope.krate().edition(semantics_scope.db),
            )
            .to_string(),
        )
    });

    let mut name = new_fn_name.to_owned();
    let mut counter = 0;
    while names_in_scope.contains(&name) {
        counter += 1;
        name = format!("{new_fn_name}{counter}")
    }
    make::name_ref(&name)
}

/// Try to guess what user wants to extract
///
/// We have basically have two cases:
/// * We want whole node, like `loop {}`, `2 + 2`, `{ let n = 1; }` exprs.
///   Then we can use `ast::Expr`
/// * We want a few statements for a block. E.g.
///   ```rust,no_run
///   fn foo() -> i32 {
///     let m = 1;
///     $0
///     let n = 2;
///     let k = 3;
///     k + n
///     $0
///   }
///   ```
///
fn extraction_target(node: &SyntaxNode, selection_range: TextRange) -> Option<FunctionBody> {
    if let Some(stmt) = ast::Stmt::cast(node.clone()) {
        return match stmt {
            ast::Stmt::Item(_) => None,
            ast::Stmt::ExprStmt(_) | ast::Stmt::LetStmt(_) => FunctionBody::from_range(
                node.parent().and_then(ast::StmtList::cast)?,
                node.text_range(),
            ),
        };
    }

    // Covering element returned the parent block of one or multiple statements that have been selected
    if let Some(stmt_list) = ast::StmtList::cast(node.clone()) {
        if let Some(block_expr) = stmt_list.syntax().parent().and_then(ast::BlockExpr::cast) {
            if block_expr.syntax().text_range() == selection_range {
                return FunctionBody::from_expr(block_expr.into());
            }
        }

        // Extract the full statements.
        return FunctionBody::from_range(stmt_list, selection_range);
    }

    let expr = ast::Expr::cast(node.clone())?;
    // A node got selected fully
    if node.text_range() == selection_range {
        return FunctionBody::from_expr(expr);
    }

    node.ancestors().find_map(ast::Expr::cast).and_then(FunctionBody::from_expr)
}

#[derive(Debug)]
struct Function {
    name: ast::NameRef,
    self_param: Option<ast::SelfParam>,
    params: Vec<Param>,
    control_flow: ControlFlow,
    ret_ty: RetType,
    body: FunctionBody,
    outliving_locals: Vec<OutlivedLocal>,
    /// Whether at least one of the container's tail expr is contained in the range we're extracting.
    contains_tail_expr: bool,
    mods: ContainerInfo,
}

#[derive(Debug)]
struct Param {
    var: Local,
    ty: hir::Type,
    move_local: bool,
    requires_mut: bool,
    is_copy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParamKind {
    Value,
    MutValue,
    SharedRef,
    MutRef,
}

#[derive(Debug)]
enum FunType {
    Unit,
    Single(hir::Type),
    Tuple(Vec<hir::Type>),
}

/// Where to put extracted function definition
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Anchor {
    /// Extract free function and put right after current top-level function
    Freestanding,
    /// Extract method and put right after current function in the impl-block
    Method,
}

// FIXME: ControlFlow and ContainerInfo both track some function modifiers, feels like these two should
// probably be merged somehow.
#[derive(Debug)]
struct ControlFlow {
    kind: Option<FlowKind>,
    is_async: bool,
    is_unsafe: bool,
}

/// The thing whose expression we are extracting from. Can be a function, const, static, const arg, ...
#[derive(Clone, Debug)]
struct ContainerInfo {
    is_const: bool,
    parent_loop: Option<SyntaxNode>,
    /// The function's return type, const's type etc.
    ret_type: Option<hir::Type>,
    generic_param_lists: Vec<ast::GenericParamList>,
    where_clauses: Vec<ast::WhereClause>,
    edition: Edition,
}

/// Control flow that is exported from extracted function
///
/// E.g.:
/// ```rust,no_run
/// loop {
///     $0
///     if 42 == 42 {
///         break;
///     }
///     $0
/// }
/// ```
#[derive(Debug, Clone)]
enum FlowKind {
    /// Return with value (`return $expr;`)
    Return(Option<ast::Expr>),
    Try {
        kind: TryKind,
    },
    /// Break with label and value (`break 'label $expr;`)
    Break(Option<ast::Lifetime>, Option<ast::Expr>),
    /// Continue with label (`continue 'label;`)
    Continue(Option<ast::Lifetime>),
}

#[derive(Debug, Clone)]
enum TryKind {
    Option,
    Result { ty: hir::Type },
}

#[derive(Debug)]
enum RetType {
    Expr(hir::Type),
    Stmt,
}

impl RetType {
    fn is_unit(&self) -> bool {
        match self {
            RetType::Expr(ty) => ty.is_unit(),
            RetType::Stmt => true,
        }
    }
}

/// Semantically same as `ast::Expr`, but preserves identity when using only part of the Block
/// This is the future function body, the part that is being extracted.
#[derive(Debug)]
enum FunctionBody {
    Expr(ast::Expr),
    Span { parent: ast::StmtList, elements: RangeInclusive<SyntaxElement>, text_range: TextRange },
}

#[derive(Debug)]
struct OutlivedLocal {
    local: Local,
    mut_usage_outside_body: bool,
}

/// Container of local variable usages
///
/// Semantically same as `UsageSearchResult`, but provides more convenient interface
struct LocalUsages(ide_db::search::UsageSearchResult);

impl LocalUsages {
    fn find_local_usages(ctx: &AssistContext<'_>, var: Local) -> Self {
        Self(
            Definition::Local(var)
                .usages(&ctx.sema)
                .in_scope(&SearchScope::single_file(ctx.file_id()))
                .all(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &FileReference> + '_ {
        self.0.iter().flat_map(|(_, rs)| rs)
    }
}

impl Function {
    fn return_type(&self, ctx: &AssistContext<'_>) -> FunType {
        match &self.ret_ty {
            RetType::Expr(ty) if ty.is_unit() => FunType::Unit,
            RetType::Expr(ty) => FunType::Single(ty.clone()),
            RetType::Stmt => match self.outliving_locals.as_slice() {
                [] => FunType::Unit,
                [var] => FunType::Single(var.local.ty(ctx.db())),
                vars => {
                    let types = vars.iter().map(|v| v.local.ty(ctx.db())).collect();
                    FunType::Tuple(types)
                }
            },
        }
    }

    fn self_param_adt(&self, ctx: &AssistContext<'_>) -> Option<ast::Adt> {
        let self_param = self.self_param.as_ref()?;
        let def = ctx.sema.to_def(self_param)?;
        let adt = def.ty(ctx.db()).strip_references().as_adt()?;
        let InFile { file_id: _, value } = adt.source(ctx.db())?;
        Some(value)
    }
}

impl ParamKind {
    fn is_ref(&self) -> bool {
        matches!(self, ParamKind::SharedRef | ParamKind::MutRef)
    }
}

impl Param {
    fn kind(&self) -> ParamKind {
        match (self.move_local, self.requires_mut, self.is_copy) {
            (false, true, _) => ParamKind::MutRef,
            (false, false, false) => ParamKind::SharedRef,
            (true, true, _) => ParamKind::MutValue,
            (_, false, _) => ParamKind::Value,
        }
    }

    fn to_arg(&self, ctx: &AssistContext<'_>, edition: Edition) -> ast::Expr {
        let var = path_expr_from_local(ctx, self.var, edition);
        match self.kind() {
            ParamKind::Value | ParamKind::MutValue => var,
            ParamKind::SharedRef => make::expr_ref(var, false),
            ParamKind::MutRef => make::expr_ref(var, true),
        }
    }

    fn to_param(
        &self,
        ctx: &AssistContext<'_>,
        module: hir::Module,
        edition: Edition,
    ) -> ast::Param {
        let var = self.var.name(ctx.db()).display(ctx.db(), edition).to_string();
        let var_name = make::name(&var);
        let pat = match self.kind() {
            ParamKind::MutValue => make::ident_pat(false, true, var_name),
            ParamKind::Value | ParamKind::SharedRef | ParamKind::MutRef => {
                make::ext::simple_ident_pat(var_name)
            }
        };

        let ty = make_ty(&self.ty, ctx, module);
        let ty = match self.kind() {
            ParamKind::Value | ParamKind::MutValue => ty,
            ParamKind::SharedRef => make::ty_ref(ty, false),
            ParamKind::MutRef => make::ty_ref(ty, true),
        };

        make::param(pat.into(), ty)
    }
}

impl TryKind {
    fn of_ty(ty: hir::Type, ctx: &AssistContext<'_>, edition: Edition) -> Option<TryKind> {
        if ty.is_unknown() {
            // We favour Result for `expr?`
            return Some(TryKind::Result { ty });
        }
        let adt = ty.as_adt()?;
        let name = adt.name(ctx.db());
        // FIXME: use lang items to determine if it is std type or user defined
        //        E.g. if user happens to define type named `Option`, we would have false positive
        let name = &name.display(ctx.db(), edition).to_string();
        match name.as_str() {
            "Option" => Some(TryKind::Option),
            "Result" => Some(TryKind::Result { ty }),
            _ => None,
        }
    }
}

impl FlowKind {
    fn make_result_handler(&self, expr: Option<ast::Expr>) -> ast::Expr {
        match self {
            FlowKind::Return(_) => make::expr_return(expr),
            FlowKind::Break(label, _) => make::expr_break(label.clone(), expr),
            FlowKind::Try { .. } => {
                stdx::never!("cannot have result handler with try");
                expr.unwrap_or_else(|| make::expr_return(None))
            }
            FlowKind::Continue(label) => {
                stdx::always!(expr.is_none(), "continue with value is not possible");
                make::expr_continue(label.clone())
            }
        }
    }

    fn expr_ty(&self, ctx: &AssistContext<'_>) -> Option<hir::Type> {
        match self {
            FlowKind::Return(Some(expr)) | FlowKind::Break(_, Some(expr)) => {
                ctx.sema.type_of_expr(expr).map(TypeInfo::adjusted)
            }
            FlowKind::Try { .. } => {
                stdx::never!("try does not have defined expr_ty");
                None
            }
            _ => None,
        }
    }
}

impl FunctionBody {
    fn parent(&self) -> Option<SyntaxNode> {
        match self {
            FunctionBody::Expr(expr) => expr.syntax().parent(),
            FunctionBody::Span { parent, .. } => Some(parent.syntax().clone()),
        }
    }

    fn node(&self) -> &SyntaxNode {
        match self {
            FunctionBody::Expr(e) => e.syntax(),
            FunctionBody::Span { parent, .. } => parent.syntax(),
        }
    }

    fn extracted_from_trait_impl(&self) -> bool {
        match self.node().ancestors().find_map(ast::Impl::cast) {
            Some(c) => c.trait_().is_some(),
            None => false,
        }
    }

    fn descendants(&self) -> impl Iterator<Item = SyntaxNode> {
        match self {
            FunctionBody::Expr(expr) => expr.syntax().descendants(),
            FunctionBody::Span { parent, .. } => parent.syntax().descendants(),
        }
    }

    fn descendant_paths(&self) -> impl Iterator<Item = ast::Path> {
        self.descendants().filter_map(|node| {
            match_ast! {
                match node {
                    ast::Path(it) => Some(it),
                    _ => None
                }
            }
        })
    }

    fn from_expr(expr: ast::Expr) -> Option<Self> {
        match expr {
            ast::Expr::BreakExpr(it) => it.expr().map(Self::Expr),
            ast::Expr::ReturnExpr(it) => it.expr().map(Self::Expr),
            ast::Expr::BlockExpr(it) if !it.is_standalone() => None,
            expr => Some(Self::Expr(expr)),
        }
    }

    fn from_range(parent: ast::StmtList, selected: TextRange) -> Option<FunctionBody> {
        let full_body = parent.syntax().children_with_tokens();

        // Get all of the elements intersecting with the selection
        let mut stmts_in_selection = full_body
            .filter(|it| ast::Stmt::can_cast(it.kind()) || it.kind() == COMMENT)
            .filter(|it| selected.intersect(it.text_range()).filter(|it| !it.is_empty()).is_some());

        let first_element = stmts_in_selection.next();

        // If the tail expr is part of the selection too, make that the last element
        // Otherwise use the last stmt
        let last_element = if let Some(tail_expr) =
            parent.tail_expr().filter(|it| selected.intersect(it.syntax().text_range()).is_some())
        {
            Some(tail_expr.syntax().clone().into())
        } else {
            stmts_in_selection.last()
        };

        let elements = match (first_element, last_element) {
            (None, _) => {
                cov_mark::hit!(extract_function_empty_selection_is_not_applicable);
                return None;
            }
            (Some(first), None) => first.clone()..=first,
            (Some(first), Some(last)) => first..=last,
        };

        let text_range = elements.start().text_range().cover(elements.end().text_range());

        Some(Self::Span { parent, elements, text_range })
    }

    fn indent_level(&self) -> IndentLevel {
        match &self {
            FunctionBody::Expr(expr) => IndentLevel::from_node(expr.syntax()),
            FunctionBody::Span { parent, .. } => IndentLevel::from_node(parent.syntax()) + 1,
        }
    }

    fn tail_expr(&self) -> Option<ast::Expr> {
        match &self {
            FunctionBody::Expr(expr) => Some(expr.clone()),
            FunctionBody::Span { parent, text_range, .. } => {
                let tail_expr = parent.tail_expr()?;
                text_range.contains_range(tail_expr.syntax().text_range()).then_some(tail_expr)
            }
        }
    }

    fn walk_expr(&self, cb: &mut dyn FnMut(ast::Expr)) {
        match self {
            FunctionBody::Expr(expr) => walk_expr(expr, cb),
            FunctionBody::Span { parent, text_range, .. } => {
                parent
                    .statements()
                    .filter(|stmt| text_range.contains_range(stmt.syntax().text_range()))
                    .filter_map(|stmt| match stmt {
                        ast::Stmt::ExprStmt(expr_stmt) => expr_stmt.expr(),
                        ast::Stmt::Item(_) => None,
                        ast::Stmt::LetStmt(stmt) => stmt.initializer(),
                    })
                    .for_each(|expr| walk_expr(&expr, cb));
                if let Some(expr) = parent
                    .tail_expr()
                    .filter(|it| text_range.contains_range(it.syntax().text_range()))
                {
                    walk_expr(&expr, cb);
                }
            }
        }
    }

    fn preorder_expr(&self, cb: &mut dyn FnMut(WalkEvent<ast::Expr>) -> bool) {
        match self {
            FunctionBody::Expr(expr) => preorder_expr(expr, cb),
            FunctionBody::Span { parent, text_range, .. } => {
                parent
                    .statements()
                    .filter(|stmt| text_range.contains_range(stmt.syntax().text_range()))
                    .filter_map(|stmt| match stmt {
                        ast::Stmt::ExprStmt(expr_stmt) => expr_stmt.expr(),
                        ast::Stmt::Item(_) => None,
                        ast::Stmt::LetStmt(stmt) => stmt.initializer(),
                    })
                    .for_each(|expr| preorder_expr(&expr, cb));
                if let Some(expr) = parent
                    .tail_expr()
                    .filter(|it| text_range.contains_range(it.syntax().text_range()))
                {
                    preorder_expr(&expr, cb);
                }
            }
        }
    }

    fn walk_pat(&self, cb: &mut dyn FnMut(ast::Pat)) {
        match self {
            FunctionBody::Expr(expr) => walk_patterns_in_expr(expr, cb),
            FunctionBody::Span { parent, text_range, .. } => {
                parent
                    .statements()
                    .filter(|stmt| text_range.contains_range(stmt.syntax().text_range()))
                    .for_each(|stmt| match stmt {
                        ast::Stmt::ExprStmt(expr_stmt) => {
                            if let Some(expr) = expr_stmt.expr() {
                                walk_patterns_in_expr(&expr, cb)
                            }
                        }
                        ast::Stmt::Item(_) => (),
                        ast::Stmt::LetStmt(stmt) => {
                            if let Some(pat) = stmt.pat() {
                                walk_pat(&pat, cb);
                            }
                            if let Some(expr) = stmt.initializer() {
                                walk_patterns_in_expr(&expr, cb);
                            }
                        }
                    });
                if let Some(expr) = parent
                    .tail_expr()
                    .filter(|it| text_range.contains_range(it.syntax().text_range()))
                {
                    walk_patterns_in_expr(&expr, cb);
                }
            }
        }
    }

    fn text_range(&self) -> TextRange {
        match self {
            FunctionBody::Expr(expr) => expr.syntax().text_range(),
            &FunctionBody::Span { text_range, .. } => text_range,
        }
    }

    fn contains_range(&self, range: TextRange) -> bool {
        self.text_range().contains_range(range)
    }

    fn precedes_range(&self, range: TextRange) -> bool {
        self.text_range().end() <= range.start()
    }

    fn contains_node(&self, node: &SyntaxNode) -> bool {
        self.contains_range(node.text_range())
    }
}

impl FunctionBody {
    /// Analyzes a function body, returning the used local variables that are referenced in it as well as
    /// whether it contains an await expression.
    fn analyze(
        &self,
        sema: &Semantics<'_, RootDatabase>,
    ) -> (FxIndexSet<Local>, Option<ast::SelfParam>) {
        let mut self_param = None;
        let mut res = FxIndexSet::default();
        let mut add_name_if_local = |name_ref: Option<_>| {
            let local_ref =
                match name_ref.and_then(|name_ref| NameRefClass::classify(sema, &name_ref)) {
                    Some(
                        NameRefClass::Definition(Definition::Local(local_ref))
                        | NameRefClass::FieldShorthand { local_ref, field_ref: _ },
                    ) => local_ref,
                    _ => return,
                };
            let InFile { file_id, value } = local_ref.primary_source(sema.db).source;
            // locals defined inside macros are not relevant to us
            if !file_id.is_macro() {
                match value {
                    Either::Right(it) => {
                        self_param.replace(it);
                    }
                    Either::Left(_) => {
                        res.insert(local_ref);
                    }
                }
            }
        };
        self.walk_expr(&mut |expr| match expr {
            ast::Expr::PathExpr(path_expr) => {
                add_name_if_local(path_expr.path().and_then(|it| it.as_single_name_ref()))
            }
            ast::Expr::ClosureExpr(closure_expr) => {
                if let Some(body) = closure_expr.body() {
                    body.syntax()
                        .descendants()
                        .map(ast::NameRef::cast)
                        .for_each(&mut add_name_if_local);
                }
            }
            ast::Expr::MacroExpr(expr) => {
                if let Some(tt) = expr.macro_call().and_then(|call| call.token_tree()) {
                    tt.syntax()
                        .descendants_with_tokens()
                        .filter_map(SyntaxElement::into_token)
                        .filter(|it| matches!(it.kind(), SyntaxKind::IDENT | T![self]))
                        .flat_map(|t| sema.descend_into_macros_exact(t))
                        .for_each(|t| add_name_if_local(t.parent().and_then(ast::NameRef::cast)));
                }
            }
            _ => (),
        });
        (res, self_param)
    }

    fn analyze_container(
        &self,
        sema: &Semantics<'_, RootDatabase>,
        edition: Edition,
    ) -> Option<(ContainerInfo, bool)> {
        let mut ancestors = self.parent()?.ancestors();
        let infer_expr_opt = |expr| sema.type_of_expr(&expr?).map(TypeInfo::adjusted);
        let mut parent_loop = None;
        let mut set_parent_loop = |loop_: &dyn ast::HasLoopBody| {
            if loop_
                .loop_body()
                .map_or(false, |it| it.syntax().text_range().contains_range(self.text_range()))
            {
                parent_loop.get_or_insert(loop_.syntax().clone());
            }
        };

        let (is_const, expr, ty) = loop {
            let anc = ancestors.next()?;
            break match_ast! {
                match anc {
                    ast::ClosureExpr(closure) => (false, closure.body(), infer_expr_opt(closure.body())),
                    ast::BlockExpr(block_expr) => {
                        let (constness, block) = match block_expr.modifier() {
                            Some(ast::BlockModifier::Const(_)) => (true, block_expr),
                            Some(ast::BlockModifier::Try(_)) => (false, block_expr),
                            Some(ast::BlockModifier::Label(label)) if label.lifetime().is_some() => (false, block_expr),
                            _ => continue,
                        };
                        let expr = Some(ast::Expr::BlockExpr(block));
                        (constness, expr.clone(), infer_expr_opt(expr))
                    },
                    ast::Fn(fn_) => {
                        let func = sema.to_def(&fn_)?;
                        let mut ret_ty = func.ret_type(sema.db);
                        if func.is_async(sema.db) {
                            if let Some(async_ret) = func.async_ret_type(sema.db) {
                                ret_ty = async_ret;
                            }
                        }
                        (fn_.const_token().is_some(), fn_.body().map(ast::Expr::BlockExpr), Some(ret_ty))
                    },
                    ast::Static(statik) => {
                        (true, statik.body(), Some(sema.to_def(&statik)?.ty(sema.db)))
                    },
                    ast::ConstArg(ca) => {
                        (true, ca.expr(), infer_expr_opt(ca.expr()))
                    },
                    ast::Const(konst) => {
                        (true, konst.body(), Some(sema.to_def(&konst)?.ty(sema.db)))
                    },
                    ast::ConstParam(cp) => {
                        (true, cp.default_val()?.expr(), Some(sema.to_def(&cp)?.ty(sema.db)))
                    },
                    ast::ConstBlockPat(cbp) => {
                        let expr = cbp.block_expr().map(ast::Expr::BlockExpr);
                        (true, expr.clone(), infer_expr_opt(expr))
                    },
                    ast::Variant(__) => return None,
                    ast::Meta(__) => return None,
                    ast::LoopExpr(it) => {
                        set_parent_loop(&it);
                        continue;
                    },
                    ast::ForExpr(it) => {
                        set_parent_loop(&it);
                        continue;
                    },
                    ast::WhileExpr(it) => {
                        set_parent_loop(&it);
                        continue;
                    },
                    _ => continue,
                }
            };
        };

        let expr = expr?;
        let contains_tail_expr = if let Some(body_tail) = self.tail_expr() {
            let mut contains_tail_expr = false;
            let tail_expr_range = body_tail.syntax().text_range();
            for_each_tail_expr(&expr, &mut |e| {
                if tail_expr_range.contains_range(e.syntax().text_range()) {
                    contains_tail_expr = true;
                }
            });
            contains_tail_expr
        } else {
            false
        };

        let parent = self.parent()?;
        let parents = generic_parents(&parent);
        let generic_param_lists = parents.iter().filter_map(|it| it.generic_param_list()).collect();
        let where_clauses = parents.iter().filter_map(|it| it.where_clause()).collect();

        Some((
            ContainerInfo {
                is_const,
                parent_loop,
                ret_type: ty,
                generic_param_lists,
                where_clauses,
                edition,
            },
            contains_tail_expr,
        ))
    }

    fn return_ty(&self, ctx: &AssistContext<'_>) -> Option<RetType> {
        match self.tail_expr() {
            Some(expr) => ctx.sema.type_of_expr(&expr).map(TypeInfo::original).map(RetType::Expr),
            None => Some(RetType::Stmt),
        }
    }

    /// Local variables defined inside `body` that are accessed outside of it
    fn ret_values<'a>(
        &self,
        ctx: &'a AssistContext<'_>,
        parent: &SyntaxNode,
    ) -> impl Iterator<Item = OutlivedLocal> + 'a {
        let parent = parent.clone();
        let range = self.text_range();
        locals_defined_in_body(&ctx.sema, self)
            .into_iter()
            .filter_map(move |local| local_outlives_body(ctx, range, local, &parent))
    }

    /// Analyses the function body for external control flow.
    fn external_control_flow(
        &self,
        ctx: &AssistContext<'_>,
        container_info: &ContainerInfo,
    ) -> Option<ControlFlow> {
        let mut ret_expr = None;
        let mut try_expr = None;
        let mut break_expr = None;
        let mut continue_expr = None;
        let mut is_async = false;
        let mut _is_unsafe = false;

        let mut unsafe_depth = 0;
        let mut loop_depth = 0;

        self.preorder_expr(&mut |expr| {
            let expr = match expr {
                WalkEvent::Enter(e) => e,
                WalkEvent::Leave(expr) => {
                    match expr {
                        ast::Expr::LoopExpr(_)
                        | ast::Expr::ForExpr(_)
                        | ast::Expr::WhileExpr(_) => loop_depth -= 1,
                        ast::Expr::BlockExpr(block_expr) if block_expr.unsafe_token().is_some() => {
                            unsafe_depth -= 1
                        }
                        _ => (),
                    }
                    return false;
                }
            };
            match expr {
                ast::Expr::LoopExpr(_) | ast::Expr::ForExpr(_) | ast::Expr::WhileExpr(_) => {
                    loop_depth += 1;
                }
                ast::Expr::BlockExpr(block_expr) if block_expr.unsafe_token().is_some() => {
                    unsafe_depth += 1
                }
                ast::Expr::ReturnExpr(it) => {
                    ret_expr = Some(it);
                }
                ast::Expr::TryExpr(it) => {
                    try_expr = Some(it);
                }
                ast::Expr::BreakExpr(it) if loop_depth == 0 => {
                    break_expr = Some(it);
                }
                ast::Expr::ContinueExpr(it) if loop_depth == 0 => {
                    continue_expr = Some(it);
                }
                ast::Expr::AwaitExpr(_) => is_async = true,
                // FIXME: Do unsafe analysis on expression, sem highlighting knows this so we should be able
                // to just lift that out of there
                // expr if unsafe_depth ==0 && expr.is_unsafe => is_unsafe = true,
                _ => {}
            }
            false
        });

        let kind = match (try_expr, ret_expr, break_expr, continue_expr) {
            (Some(_), _, None, None) => {
                let ret_ty = container_info.ret_type.clone()?;
                let kind = TryKind::of_ty(ret_ty, ctx, container_info.edition)?;

                Some(FlowKind::Try { kind })
            }
            (Some(_), _, _, _) => {
                cov_mark::hit!(external_control_flow_try_and_bc);
                return None;
            }
            (None, Some(r), None, None) => Some(FlowKind::Return(r.expr())),
            (None, Some(_), _, _) => {
                cov_mark::hit!(external_control_flow_return_and_bc);
                return None;
            }
            (None, None, Some(_), Some(_)) => {
                cov_mark::hit!(external_control_flow_break_and_continue);
                return None;
            }
            (None, None, Some(b), None) => Some(FlowKind::Break(b.lifetime(), b.expr())),
            (None, None, None, Some(c)) => Some(FlowKind::Continue(c.lifetime())),
            (None, None, None, None) => None,
        };

        Some(ControlFlow { kind, is_async, is_unsafe: _is_unsafe })
    }

    /// find variables that should be extracted as params
    ///
    /// Computes additional info that affects param type and mutability
    fn extracted_function_params(
        &self,
        ctx: &AssistContext<'_>,
        container_info: &ContainerInfo,
        locals: impl Iterator<Item = Local>,
    ) -> Vec<Param> {
        locals
            .map(|local| (local, local.primary_source(ctx.db())))
            .filter(|(_, src)| is_defined_outside_of_body(ctx, self, src))
            .filter_map(|(local, src)| match src.into_ident_pat() {
                Some(src) => Some((local, src)),
                None => {
                    stdx::never!(false, "Local::is_self returned false, but source is SelfParam");
                    None
                }
            })
            .map(|(var, src)| {
                let usages = LocalUsages::find_local_usages(ctx, var);
                let ty = var.ty(ctx.db());

                let defined_outside_parent_loop = container_info
                    .parent_loop
                    .as_ref()
                    .map_or(true, |it| it.text_range().contains_range(src.syntax().text_range()));

                let is_copy = ty.is_copy(ctx.db());
                let has_usages = self.has_usages_after_body(&usages);
                let requires_mut =
                    !ty.is_mutable_reference() && has_exclusive_usages(ctx, &usages, self);
                // We can move the value into the function call if it's not used after the call,
                // if the var is not used but defined outside a loop we are extracting from we can't move it either
                // as the function will reuse it in the next iteration.
                let move_local = (!has_usages && defined_outside_parent_loop) || ty.is_reference();
                Param { var, ty, move_local, requires_mut, is_copy }
            })
            .collect()
    }

    fn has_usages_after_body(&self, usages: &LocalUsages) -> bool {
        usages.iter().any(|reference| self.precedes_range(reference.range))
    }
}

enum GenericParent {
    Fn(ast::Fn),
    Impl(ast::Impl),
    Trait(ast::Trait),
}

impl GenericParent {
    fn generic_param_list(&self) -> Option<ast::GenericParamList> {
        match self {
            GenericParent::Fn(fn_) => fn_.generic_param_list(),
            GenericParent::Impl(impl_) => impl_.generic_param_list(),
            GenericParent::Trait(trait_) => trait_.generic_param_list(),
        }
    }

    fn where_clause(&self) -> Option<ast::WhereClause> {
        match self {
            GenericParent::Fn(fn_) => fn_.where_clause(),
            GenericParent::Impl(impl_) => impl_.where_clause(),
            GenericParent::Trait(trait_) => trait_.where_clause(),
        }
    }
}

/// Search `parent`'s ancestors for items with potentially applicable generic parameters
fn generic_parents(parent: &SyntaxNode) -> Vec<GenericParent> {
    let mut list = Vec::new();
    if let Some(parent_item) = parent.ancestors().find_map(ast::Item::cast) {
        if let ast::Item::Fn(ref fn_) = parent_item {
            if let Some(parent_parent) =
                parent_item.syntax().parent().and_then(|it| it.parent()).and_then(ast::Item::cast)
            {
                match parent_parent {
                    ast::Item::Impl(impl_) => list.push(GenericParent::Impl(impl_)),
                    ast::Item::Trait(trait_) => list.push(GenericParent::Trait(trait_)),
                    _ => (),
                }
            }
            list.push(GenericParent::Fn(fn_.clone()));
        }
    }
    list
}

/// checks if relevant var is used with `&mut` access inside body
fn has_exclusive_usages(
    ctx: &AssistContext<'_>,
    usages: &LocalUsages,
    body: &FunctionBody,
) -> bool {
    usages
        .iter()
        .filter(|reference| body.contains_range(reference.range))
        .any(|reference| reference_is_exclusive(reference, body, ctx))
}

/// checks if this reference requires `&mut` access inside node
fn reference_is_exclusive(
    reference: &FileReference,
    node: &dyn HasTokenAtOffset,
    ctx: &AssistContext<'_>,
) -> bool {
    // FIXME: this quite an incorrect way to go about doing this :-)
    // `FileReference` is an IDE-type --- it encapsulates data communicated to the human,
    // but doesn't necessary fully reflect all the intricacies of the underlying language semantics
    // The correct approach here would be to expose this entire analysis as a method on some hir
    // type. Something like `body.free_variables(statement_range)`.

    // we directly modify variable with set: `n = 0`, `n += 1`
    if reference.category.contains(ReferenceCategory::WRITE) {
        return true;
    }

    // we take `&mut` reference to variable: `&mut v`
    let path = match path_element_of_reference(node, reference) {
        Some(path) => path,
        None => return false,
    };

    expr_require_exclusive_access(ctx, &path).unwrap_or(false)
}

/// checks if this expr requires `&mut` access, recurses on field access
fn expr_require_exclusive_access(ctx: &AssistContext<'_>, expr: &ast::Expr) -> Option<bool> {
    if let ast::Expr::MacroExpr(_) = expr {
        // FIXME: expand macro and check output for mutable usages of the variable?
        return None;
    }

    let parent = expr.syntax().parent()?;

    if let Some(bin_expr) = ast::BinExpr::cast(parent.clone()) {
        if matches!(bin_expr.op_kind()?, ast::BinaryOp::Assignment { .. }) {
            return Some(bin_expr.lhs()?.syntax() == expr.syntax());
        }
        return Some(false);
    }

    if let Some(ref_expr) = ast::RefExpr::cast(parent.clone()) {
        return Some(ref_expr.mut_token().is_some());
    }

    if let Some(method_call) = ast::MethodCallExpr::cast(parent.clone()) {
        let func = ctx.sema.resolve_method_call(&method_call)?;
        let self_param = func.self_param(ctx.db())?;
        let access = self_param.access(ctx.db());

        return Some(matches!(access, hir::Access::Exclusive));
    }

    if let Some(field) = ast::FieldExpr::cast(parent) {
        return expr_require_exclusive_access(ctx, &field.into());
    }

    Some(false)
}

trait HasTokenAtOffset {
    fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken>;
}

impl HasTokenAtOffset for SyntaxNode {
    fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
        SyntaxNode::token_at_offset(self, offset)
    }
}

impl HasTokenAtOffset for FunctionBody {
    fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
        match self {
            FunctionBody::Expr(expr) => expr.syntax().token_at_offset(offset),
            FunctionBody::Span { parent, text_range, .. } => {
                match parent.syntax().token_at_offset(offset) {
                    TokenAtOffset::None => TokenAtOffset::None,
                    TokenAtOffset::Single(t) => {
                        if text_range.contains_range(t.text_range()) {
                            TokenAtOffset::Single(t)
                        } else {
                            TokenAtOffset::None
                        }
                    }
                    TokenAtOffset::Between(a, b) => {
                        match (
                            text_range.contains_range(a.text_range()),
                            text_range.contains_range(b.text_range()),
                        ) {
                            (true, true) => TokenAtOffset::Between(a, b),
                            (true, false) => TokenAtOffset::Single(a),
                            (false, true) => TokenAtOffset::Single(b),
                            (false, false) => TokenAtOffset::None,
                        }
                    }
                }
            }
        }
    }
}

/// find relevant `ast::Expr` for reference
///
/// # Preconditions
///
/// `node` must cover `reference`, that is `node.text_range().contains_range(reference.range)`
fn path_element_of_reference(
    node: &dyn HasTokenAtOffset,
    reference: &FileReference,
) -> Option<ast::Expr> {
    let token = node.token_at_offset(reference.range.start()).right_biased().or_else(|| {
        stdx::never!(false, "cannot find token at variable usage: {:?}", reference);
        None
    })?;
    let path = token.parent_ancestors().find_map(ast::Expr::cast).or_else(|| {
        stdx::never!(false, "cannot find path parent of variable usage: {:?}", token);
        None
    })?;
    stdx::always!(
        matches!(path, ast::Expr::PathExpr(_) | ast::Expr::MacroExpr(_)),
        "unexpected expression type for variable usage: {:?}",
        path
    );
    Some(path)
}

/// list local variables defined inside `body`
fn locals_defined_in_body(
    sema: &Semantics<'_, RootDatabase>,
    body: &FunctionBody,
) -> FxIndexSet<Local> {
    // FIXME: this doesn't work well with macros
    //        see https://github.com/rust-lang/rust-analyzer/pull/7535#discussion_r570048550
    let mut res = FxIndexSet::default();
    body.walk_pat(&mut |pat| {
        if let ast::Pat::IdentPat(pat) = pat {
            if let Some(local) = sema.to_def(&pat) {
                res.insert(local);
            }
        }
    });
    res
}

/// Returns usage details if local variable is used after(outside of) body
fn local_outlives_body(
    ctx: &AssistContext<'_>,
    body_range: TextRange,
    local: Local,
    parent: &SyntaxNode,
) -> Option<OutlivedLocal> {
    let usages = LocalUsages::find_local_usages(ctx, local);
    let mut has_mut_usages = false;
    let mut any_outlives = false;
    for usage in usages.iter() {
        if body_range.end() <= usage.range.start() {
            has_mut_usages |= reference_is_exclusive(usage, parent, ctx);
            any_outlives |= true;
            if has_mut_usages {
                break; // no need to check more elements we have all the info we wanted
            }
        }
    }
    if !any_outlives {
        return None;
    }
    Some(OutlivedLocal { local, mut_usage_outside_body: has_mut_usages })
}

/// checks if the relevant local was defined before(outside of) body
fn is_defined_outside_of_body(
    ctx: &AssistContext<'_>,
    body: &FunctionBody,
    src: &LocalSource,
) -> bool {
    src.original_file(ctx.db()) == ctx.file_id() && !body.contains_node(src.syntax())
}

/// find where to put extracted function definition
///
/// Function should be put right after returned node
fn node_to_insert_after(body: &FunctionBody, anchor: Anchor) -> Option<SyntaxNode> {
    let node = body.node();
    let mut ancestors = node.ancestors().peekable();
    let mut last_ancestor = None;
    while let Some(next_ancestor) = ancestors.next() {
        match next_ancestor.kind() {
            SyntaxKind::SOURCE_FILE => break,
            SyntaxKind::IMPL => {
                if body.extracted_from_trait_impl() && matches!(anchor, Anchor::Method) {
                    let impl_node = find_non_trait_impl(&next_ancestor);
                    if let target_node @ Some(_) = impl_node.as_ref().and_then(last_impl_member) {
                        return target_node;
                    }
                }
            }
            SyntaxKind::ITEM_LIST if !matches!(anchor, Anchor::Freestanding) => continue,
            SyntaxKind::ITEM_LIST => {
                if ancestors.peek().map(SyntaxNode::kind) == Some(SyntaxKind::MODULE) {
                    break;
                }
            }
            SyntaxKind::ASSOC_ITEM_LIST if !matches!(anchor, Anchor::Method) => continue,
            SyntaxKind::ASSOC_ITEM_LIST if body.extracted_from_trait_impl() => continue,
            SyntaxKind::ASSOC_ITEM_LIST => {
                if ancestors.peek().map(SyntaxNode::kind) == Some(SyntaxKind::IMPL) {
                    break;
                }
            }
            _ => (),
        }
        last_ancestor = Some(next_ancestor);
    }
    last_ancestor
}

fn find_non_trait_impl(trait_impl: &SyntaxNode) -> Option<ast::Impl> {
    let as_impl = ast::Impl::cast(trait_impl.clone())?;
    let impl_type = Some(impl_type_name(&as_impl)?);

    let siblings = trait_impl.parent()?.children();
    siblings
        .filter_map(ast::Impl::cast)
        .find(|s| impl_type_name(s) == impl_type && !is_trait_impl(s))
}

fn last_impl_member(impl_node: &ast::Impl) -> Option<SyntaxNode> {
    let last_child = impl_node.assoc_item_list()?.assoc_items().last()?;
    Some(last_child.syntax().clone())
}

fn is_trait_impl(node: &ast::Impl) -> bool {
    node.trait_().is_some()
}

fn impl_type_name(impl_node: &ast::Impl) -> Option<String> {
    Some(impl_node.self_ty()?.to_string())
}

/// Fixes up the call site before the target expressions are replaced with the call expression
fn fixup_call_site(builder: &mut SourceChangeBuilder, body: &FunctionBody) {
    let parent_match_arm = body.parent().and_then(ast::MatchArm::cast);

    if let Some(parent_match_arm) = parent_match_arm {
        if parent_match_arm.comma_token().is_none() {
            let parent_match_arm = builder.make_mut(parent_match_arm);
            ted::append_child_raw(parent_match_arm.syntax(), make::token(T![,]));
        }
    }
}

fn make_call(ctx: &AssistContext<'_>, fun: &Function, indent: IndentLevel) -> SyntaxNode {
    let ret_ty = fun.return_type(ctx);

    let args = make::arg_list(fun.params.iter().map(|param| param.to_arg(ctx, fun.mods.edition)));
    let name = fun.name.clone();
    let mut call_expr = if fun.self_param.is_some() {
        let self_arg = make::expr_path(make::ext::ident_path("self"));
        make::expr_method_call(self_arg, name, args)
    } else {
        let func = make::expr_path(make::path_unqualified(make::path_segment(name)));
        make::expr_call(func, args)
    };

    let handler = FlowHandler::from_ret_ty(fun, &ret_ty);

    if fun.control_flow.is_async {
        call_expr = make::expr_await(call_expr);
    }

    let expr = handler.make_call_expr(call_expr).clone_for_update();
    expr.indent(indent);

    let outliving_bindings = match fun.outliving_locals.as_slice() {
        [] => None,
        [var] => {
            let name = var.local.name(ctx.db());
            let name = make::name(&name.display(ctx.db(), fun.mods.edition).to_string());
            Some(ast::Pat::IdentPat(make::ident_pat(false, var.mut_usage_outside_body, name)))
        }
        vars => {
            let binding_pats = vars.iter().map(|var| {
                let name = var.local.name(ctx.db());
                let name = make::name(&name.display(ctx.db(), fun.mods.edition).to_string());
                make::ident_pat(false, var.mut_usage_outside_body, name).into()
            });
            Some(ast::Pat::TuplePat(make::tuple_pat(binding_pats)))
        }
    };

    let parent_match_arm = fun.body.parent().and_then(ast::MatchArm::cast);

    if let Some(bindings) = outliving_bindings {
        // with bindings that outlive it
        make::let_stmt(bindings, None, Some(expr)).syntax().clone_for_update()
    } else if parent_match_arm.as_ref().is_some() {
        // as a tail expr for a match arm
        expr.syntax().clone()
    } else if parent_match_arm.as_ref().is_none()
        && fun.ret_ty.is_unit()
        && (!fun.outliving_locals.is_empty() || !expr.is_block_like())
    {
        // as an expr stmt
        make::expr_stmt(expr).syntax().clone_for_update()
    } else {
        // as a tail expr, or a block
        expr.syntax().clone()
    }
}

enum FlowHandler {
    None,
    Try { kind: TryKind },
    If { action: FlowKind },
    IfOption { action: FlowKind },
    MatchOption { none: FlowKind },
    MatchResult { err: FlowKind },
}

impl FlowHandler {
    fn from_ret_ty(fun: &Function, ret_ty: &FunType) -> FlowHandler {
        if fun.contains_tail_expr {
            return FlowHandler::None;
        }
        let Some(action) = fun.control_flow.kind.clone() else {
            return FlowHandler::None;
        };

        if let FunType::Unit = ret_ty {
            match action {
                FlowKind::Return(None) | FlowKind::Break(_, None) | FlowKind::Continue(_) => {
                    FlowHandler::If { action }
                }
                FlowKind::Return(_) | FlowKind::Break(_, _) => FlowHandler::IfOption { action },
                FlowKind::Try { kind } => FlowHandler::Try { kind },
            }
        } else {
            match action {
                FlowKind::Return(None) | FlowKind::Break(_, None) | FlowKind::Continue(_) => {
                    FlowHandler::MatchOption { none: action }
                }
                FlowKind::Return(_) | FlowKind::Break(_, _) => {
                    FlowHandler::MatchResult { err: action }
                }
                FlowKind::Try { kind } => FlowHandler::Try { kind },
            }
        }
    }

    fn make_call_expr(&self, call_expr: ast::Expr) -> ast::Expr {
        match self {
            FlowHandler::None => call_expr,
            FlowHandler::Try { kind: _ } => make::expr_try(call_expr),
            FlowHandler::If { action } => {
                let action = action.make_result_handler(None);
                let stmt = make::expr_stmt(action);
                let block = make::block_expr(iter::once(stmt.into()), None);
                let controlflow_break_path = make::path_from_text("ControlFlow::Break");
                let condition = make::expr_let(
                    make::tuple_struct_pat(
                        controlflow_break_path,
                        iter::once(make::wildcard_pat().into()),
                    )
                    .into(),
                    call_expr,
                );
                make::expr_if(condition.into(), block, None)
            }
            FlowHandler::IfOption { action } => {
                let path = make::ext::ident_path("Some");
                let value_pat = make::ext::simple_ident_pat(make::name("value"));
                let pattern = make::tuple_struct_pat(path, iter::once(value_pat.into()));
                let cond = make::expr_let(pattern.into(), call_expr);
                let value = make::expr_path(make::ext::ident_path("value"));
                let action_expr = action.make_result_handler(Some(value));
                let action_stmt = make::expr_stmt(action_expr);
                let then = make::block_expr(iter::once(action_stmt.into()), None);
                make::expr_if(cond.into(), then, None)
            }
            FlowHandler::MatchOption { none } => {
                let some_name = "value";

                let some_arm = {
                    let path = make::ext::ident_path("Some");
                    let value_pat = make::ext::simple_ident_pat(make::name(some_name));
                    let pat = make::tuple_struct_pat(path, iter::once(value_pat.into()));
                    let value = make::expr_path(make::ext::ident_path(some_name));
                    make::match_arm(iter::once(pat.into()), None, value)
                };
                let none_arm = {
                    let path = make::ext::ident_path("None");
                    let pat = make::path_pat(path);
                    make::match_arm(iter::once(pat), None, none.make_result_handler(None))
                };
                let arms = make::match_arm_list(vec![some_arm, none_arm]);
                make::expr_match(call_expr, arms)
            }
            FlowHandler::MatchResult { err } => {
                let ok_name = "value";
                let err_name = "value";

                let ok_arm = {
                    let path = make::ext::ident_path("Ok");
                    let value_pat = make::ext::simple_ident_pat(make::name(ok_name));
                    let pat = make::tuple_struct_pat(path, iter::once(value_pat.into()));
                    let value = make::expr_path(make::ext::ident_path(ok_name));
                    make::match_arm(iter::once(pat.into()), None, value)
                };
                let err_arm = {
                    let path = make::ext::ident_path("Err");
                    let value_pat = make::ext::simple_ident_pat(make::name(err_name));
                    let pat = make::tuple_struct_pat(path, iter::once(value_pat.into()));
                    let value = make::expr_path(make::ext::ident_path(err_name));
                    make::match_arm(
                        iter::once(pat.into()),
                        None,
                        err.make_result_handler(Some(value)),
                    )
                };
                let arms = make::match_arm_list(vec![ok_arm, err_arm]);
                make::expr_match(call_expr, arms)
            }
        }
    }
}

fn path_expr_from_local(ctx: &AssistContext<'_>, var: Local, edition: Edition) -> ast::Expr {
    let name = var.name(ctx.db()).display(ctx.db(), edition).to_string();
    make::expr_path(make::ext::ident_path(&name))
}

fn format_function(
    ctx: &AssistContext<'_>,
    module: hir::Module,
    fun: &Function,
    old_indent: IndentLevel,
) -> ast::Fn {
    let fun_name = make::name(&fun.name.text());
    let params = fun.make_param_list(ctx, module, fun.mods.edition);
    let ret_ty = fun.make_ret_ty(ctx, module);
    let body = make_body(ctx, old_indent, fun);
    let (generic_params, where_clause) = make_generic_params_and_where_clause(ctx, fun);

    make::fn_(
        None,
        fun_name,
        generic_params,
        where_clause,
        params,
        body,
        ret_ty,
        fun.control_flow.is_async,
        fun.mods.is_const,
        fun.control_flow.is_unsafe,
        false,
    )
}

fn make_generic_params_and_where_clause(
    ctx: &AssistContext<'_>,
    fun: &Function,
) -> (Option<ast::GenericParamList>, Option<ast::WhereClause>) {
    let used_type_params = fun.type_params(ctx);

    let generic_param_list = make_generic_param_list(ctx, fun, &used_type_params);
    let where_clause = make_where_clause(ctx, fun, &used_type_params);

    (generic_param_list, where_clause)
}

fn make_generic_param_list(
    ctx: &AssistContext<'_>,
    fun: &Function,
    used_type_params: &[TypeParam],
) -> Option<ast::GenericParamList> {
    let mut generic_params = fun
        .mods
        .generic_param_lists
        .iter()
        .flat_map(|parent_params| {
            parent_params
                .generic_params()
                .filter(|param| param_is_required(ctx, param, used_type_params))
        })
        .peekable();

    if generic_params.peek().is_some() {
        Some(make::generic_param_list(generic_params))
    } else {
        None
    }
}

fn param_is_required(
    ctx: &AssistContext<'_>,
    param: &ast::GenericParam,
    used_type_params: &[TypeParam],
) -> bool {
    match param {
        ast::GenericParam::ConstParam(_) | ast::GenericParam::LifetimeParam(_) => false,
        ast::GenericParam::TypeParam(type_param) => match &ctx.sema.to_def(type_param) {
            Some(def) => used_type_params.contains(def),
            _ => false,
        },
    }
}

fn make_where_clause(
    ctx: &AssistContext<'_>,
    fun: &Function,
    used_type_params: &[TypeParam],
) -> Option<ast::WhereClause> {
    let mut predicates = fun
        .mods
        .where_clauses
        .iter()
        .flat_map(|parent_where_clause| {
            parent_where_clause
                .predicates()
                .filter(|pred| pred_is_required(ctx, pred, used_type_params))
        })
        .peekable();

    if predicates.peek().is_some() {
        Some(make::where_clause(predicates))
    } else {
        None
    }
}

fn pred_is_required(
    ctx: &AssistContext<'_>,
    pred: &ast::WherePred,
    used_type_params: &[TypeParam],
) -> bool {
    match resolved_type_param(ctx, pred) {
        Some(it) => used_type_params.contains(&it),
        None => false,
    }
}

fn resolved_type_param(ctx: &AssistContext<'_>, pred: &ast::WherePred) -> Option<TypeParam> {
    let path = match pred.ty()? {
        ast::Type::PathType(path_type) => path_type.path(),
        _ => None,
    }?;

    match ctx.sema.resolve_path(&path)? {
        PathResolution::TypeParam(type_param) => Some(type_param),
        _ => None,
    }
}

impl Function {
    /// Collect all the `TypeParam`s used in the `body` and `params`.
    fn type_params(&self, ctx: &AssistContext<'_>) -> Vec<TypeParam> {
        let type_params_in_descendant_paths =
            self.body.descendant_paths().filter_map(|it| match ctx.sema.resolve_path(&it) {
                Some(PathResolution::TypeParam(type_param)) => Some(type_param),
                _ => None,
            });
        let type_params_in_params = self.params.iter().filter_map(|p| p.ty.as_type_param(ctx.db()));
        type_params_in_descendant_paths.chain(type_params_in_params).collect()
    }

    fn make_param_list(
        &self,
        ctx: &AssistContext<'_>,
        module: hir::Module,
        edition: Edition,
    ) -> ast::ParamList {
        let self_param = self.self_param.clone();
        let params = self.params.iter().map(|param| param.to_param(ctx, module, edition));
        make::param_list(self_param, params)
    }

    fn make_ret_ty(&self, ctx: &AssistContext<'_>, module: hir::Module) -> Option<ast::RetType> {
        let fun_ty = self.return_type(ctx);
        let handler = FlowHandler::from_ret_ty(self, &fun_ty);
        let ret_ty = match &handler {
            FlowHandler::None => {
                if matches!(fun_ty, FunType::Unit) {
                    return None;
                }
                fun_ty.make_ty(ctx, module)
            }
            FlowHandler::Try { kind: TryKind::Option } => {
                make::ext::ty_option(fun_ty.make_ty(ctx, module))
            }
            FlowHandler::Try { kind: TryKind::Result { ty: parent_ret_ty } } => {
                let handler_ty = parent_ret_ty
                    .type_arguments()
                    .nth(1)
                    .map(|ty| make_ty(&ty, ctx, module))
                    .unwrap_or_else(make::ty_placeholder);
                make::ext::ty_result(fun_ty.make_ty(ctx, module), handler_ty)
            }
            FlowHandler::If { .. } => make::ty("ControlFlow<()>"),
            FlowHandler::IfOption { action } => {
                let handler_ty = action
                    .expr_ty(ctx)
                    .map(|ty| make_ty(&ty, ctx, module))
                    .unwrap_or_else(make::ty_placeholder);
                make::ext::ty_option(handler_ty)
            }
            FlowHandler::MatchOption { .. } => make::ext::ty_option(fun_ty.make_ty(ctx, module)),
            FlowHandler::MatchResult { err } => {
                let handler_ty = err
                    .expr_ty(ctx)
                    .map(|ty| make_ty(&ty, ctx, module))
                    .unwrap_or_else(make::ty_placeholder);
                make::ext::ty_result(fun_ty.make_ty(ctx, module), handler_ty)
            }
        };
        Some(make::ret_type(ret_ty))
    }
}

impl FunType {
    fn make_ty(&self, ctx: &AssistContext<'_>, module: hir::Module) -> ast::Type {
        match self {
            FunType::Unit => make::ty_unit(),
            FunType::Single(ty) => make_ty(ty, ctx, module),
            FunType::Tuple(types) => match types.as_slice() {
                [] => {
                    stdx::never!("tuple type with 0 elements");
                    make::ty_unit()
                }
                [ty] => {
                    stdx::never!("tuple type with 1 element");
                    make_ty(ty, ctx, module)
                }
                types => {
                    let types = types.iter().map(|ty| make_ty(ty, ctx, module));
                    make::ty_tuple(types)
                }
            },
        }
    }
}

fn make_body(ctx: &AssistContext<'_>, old_indent: IndentLevel, fun: &Function) -> ast::BlockExpr {
    let ret_ty = fun.return_type(ctx);
    let handler = FlowHandler::from_ret_ty(fun, &ret_ty);

    let block = match &fun.body {
        FunctionBody::Expr(expr) => {
            let expr = rewrite_body_segment(ctx, &fun.params, &handler, expr.syntax());
            let expr = ast::Expr::cast(expr).expect("Body segment should be an expr");
            match expr {
                ast::Expr::BlockExpr(block) => {
                    // If the extracted expression is itself a block, there is no need to wrap it inside another block.
                    block.dedent(old_indent);
                    let elements = block.stmt_list().map_or_else(
                        || Either::Left(iter::empty()),
                        |stmt_list| {
                            let elements = stmt_list.syntax().children_with_tokens().filter_map(
                                |node_or_token| match &node_or_token {
                                    syntax::NodeOrToken::Node(node) => {
                                        ast::Stmt::cast(node.clone()).map(|_| node_or_token)
                                    }
                                    syntax::NodeOrToken::Token(token) => {
                                        ast::Comment::cast(token.clone()).map(|_| node_or_token)
                                    }
                                },
                            );
                            Either::Right(elements)
                        },
                    );
                    make::hacky_block_expr(elements, block.tail_expr())
                }
                _ => {
                    expr.reindent_to(1.into());

                    make::block_expr(Vec::new(), Some(expr))
                }
            }
        }
        FunctionBody::Span { parent, text_range, .. } => {
            let mut elements: Vec<_> = parent
                .syntax()
                .children_with_tokens()
                .filter(|it| text_range.contains_range(it.text_range()))
                .map(|it| match &it {
                    syntax::NodeOrToken::Node(n) => syntax::NodeOrToken::Node(
                        rewrite_body_segment(ctx, &fun.params, &handler, n),
                    ),
                    _ => it,
                })
                .collect();

            let mut tail_expr = match &elements.last() {
                Some(syntax::NodeOrToken::Node(node)) if ast::Expr::can_cast(node.kind()) => {
                    ast::Expr::cast(node.clone())
                }
                _ => None,
            };

            match tail_expr {
                Some(_) => {
                    elements.pop();
                }
                None => match fun.outliving_locals.as_slice() {
                    [] => {}
                    [var] => {
                        tail_expr = Some(path_expr_from_local(ctx, var.local, fun.mods.edition));
                    }
                    vars => {
                        let exprs = vars
                            .iter()
                            .map(|var| path_expr_from_local(ctx, var.local, fun.mods.edition));
                        let expr = make::expr_tuple(exprs);
                        tail_expr = Some(expr);
                    }
                },
            };

            let body_indent = IndentLevel(1);
            let elements = elements
                .into_iter()
                .map(|node_or_token| match &node_or_token {
                    syntax::NodeOrToken::Node(node) => match ast::Stmt::cast(node.clone()) {
                        Some(stmt) => {
                            stmt.reindent_to(body_indent);
                            let ast_node = stmt.syntax().clone_subtree();
                            syntax::NodeOrToken::Node(ast_node)
                        }
                        _ => node_or_token,
                    },
                    _ => node_or_token,
                })
                .collect::<Vec<SyntaxElement>>();
            if let Some(tail_expr) = &mut tail_expr {
                tail_expr.reindent_to(body_indent);
            }

            make::hacky_block_expr(elements, tail_expr)
        }
    };

    match &handler {
        FlowHandler::None => block,
        FlowHandler::Try { kind } => {
            let block = with_default_tail_expr(block, make::expr_unit());
            map_tail_expr(block, |tail_expr| {
                let constructor = match kind {
                    TryKind::Option => "Some",
                    TryKind::Result { .. } => "Ok",
                };
                let func = make::expr_path(make::ext::ident_path(constructor));
                let args = make::arg_list(iter::once(tail_expr));
                make::expr_call(func, args)
            })
        }
        FlowHandler::If { .. } => {
            let controlflow_continue = make::expr_call(
                make::expr_path(make::path_from_text("ControlFlow::Continue")),
                make::arg_list(iter::once(make::expr_unit())),
            );
            with_tail_expr(block, controlflow_continue)
        }
        FlowHandler::IfOption { .. } => {
            let none = make::expr_path(make::ext::ident_path("None"));
            with_tail_expr(block, none)
        }
        FlowHandler::MatchOption { .. } => map_tail_expr(block, |tail_expr| {
            let some = make::expr_path(make::ext::ident_path("Some"));
            let args = make::arg_list(iter::once(tail_expr));
            make::expr_call(some, args)
        }),
        FlowHandler::MatchResult { .. } => map_tail_expr(block, |tail_expr| {
            let ok = make::expr_path(make::ext::ident_path("Ok"));
            let args = make::arg_list(iter::once(tail_expr));
            make::expr_call(ok, args)
        }),
    }
}

fn map_tail_expr(block: ast::BlockExpr, f: impl FnOnce(ast::Expr) -> ast::Expr) -> ast::BlockExpr {
    let tail_expr = match block.tail_expr() {
        Some(tail_expr) => tail_expr,
        None => return block,
    };
    make::block_expr(block.statements(), Some(f(tail_expr)))
}

fn with_default_tail_expr(block: ast::BlockExpr, tail_expr: ast::Expr) -> ast::BlockExpr {
    match block.tail_expr() {
        Some(_) => block,
        None => make::block_expr(block.statements(), Some(tail_expr)),
    }
}

fn with_tail_expr(block: ast::BlockExpr, tail_expr: ast::Expr) -> ast::BlockExpr {
    let stmt_tail_opt: Option<ast::Stmt> =
        block.tail_expr().map(|expr| make::expr_stmt(expr).into());

    let mut elements: Vec<SyntaxElement> = vec![];

    block.statements().for_each(|stmt| {
        elements.push(syntax::NodeOrToken::Node(stmt.syntax().clone()));
    });

    if let Some(stmt_list) = block.stmt_list() {
        stmt_list.syntax().children_with_tokens().for_each(|node_or_token| {
            if let syntax::NodeOrToken::Token(_) = &node_or_token {
                elements.push(node_or_token)
            };
        });
    }

    if let Some(stmt_tail) = stmt_tail_opt {
        elements.push(syntax::NodeOrToken::Node(stmt_tail.syntax().clone()));
    }

    make::hacky_block_expr(elements, Some(tail_expr))
}

fn format_type(ty: &hir::Type, ctx: &AssistContext<'_>, module: hir::Module) -> String {
    ty.display_source_code(ctx.db(), module.into(), true).ok().unwrap_or_else(|| "_".to_owned())
}

fn make_ty(ty: &hir::Type, ctx: &AssistContext<'_>, module: hir::Module) -> ast::Type {
    let ty_str = format_type(ty, ctx, module);
    make::ty(&ty_str)
}

fn rewrite_body_segment(
    ctx: &AssistContext<'_>,
    params: &[Param],
    handler: &FlowHandler,
    syntax: &SyntaxNode,
) -> SyntaxNode {
    let syntax = fix_param_usages(ctx, params, syntax);
    update_external_control_flow(handler, &syntax);
    syntax
}

/// change all usages to account for added `&`/`&mut` for some params
fn fix_param_usages(ctx: &AssistContext<'_>, params: &[Param], syntax: &SyntaxNode) -> SyntaxNode {
    let mut usages_for_param: Vec<(&Param, Vec<ast::Expr>)> = Vec::new();

    let tm = TreeMutator::new(syntax);

    for param in params {
        if !param.kind().is_ref() {
            continue;
        }

        let usages = LocalUsages::find_local_usages(ctx, param.var);
        let usages = usages
            .iter()
            .filter(|reference| syntax.text_range().contains_range(reference.range))
            .filter_map(|reference| path_element_of_reference(syntax, reference))
            .map(|expr| tm.make_mut(&expr));

        usages_for_param.push((param, usages.collect()));
    }

    let res = tm.make_syntax_mut(syntax);

    for (param, usages) in usages_for_param {
        for usage in usages {
            match usage.syntax().ancestors().skip(1).find_map(ast::Expr::cast) {
                Some(ast::Expr::MethodCallExpr(_) | ast::Expr::FieldExpr(_)) => {
                    // do nothing
                }
                Some(ast::Expr::RefExpr(node))
                    if param.kind() == ParamKind::MutRef && node.mut_token().is_some() =>
                {
                    ted::replace(
                        node.syntax(),
                        node.expr().expect("RefExpr::expr() cannot be None").syntax(),
                    );
                }
                Some(ast::Expr::RefExpr(node))
                    if param.kind() == ParamKind::SharedRef && node.mut_token().is_none() =>
                {
                    ted::replace(
                        node.syntax(),
                        node.expr().expect("RefExpr::expr() cannot be None").syntax(),
                    );
                }
                Some(_) | None => {
                    let p = &make::expr_prefix(T![*], usage.clone()).clone_for_update();
                    ted::replace(usage.syntax(), p.syntax())
                }
            }
        }
    }

    res
}

fn update_external_control_flow(handler: &FlowHandler, syntax: &SyntaxNode) {
    let mut nested_loop = None;
    let mut nested_scope = None;
    for event in syntax.preorder() {
        match event {
            WalkEvent::Enter(e) => match e.kind() {
                SyntaxKind::LOOP_EXPR | SyntaxKind::WHILE_EXPR | SyntaxKind::FOR_EXPR => {
                    if nested_loop.is_none() {
                        nested_loop = Some(e.clone());
                    }
                }
                SyntaxKind::FN
                | SyntaxKind::CONST
                | SyntaxKind::STATIC
                | SyntaxKind::IMPL
                | SyntaxKind::MODULE => {
                    if nested_scope.is_none() {
                        nested_scope = Some(e.clone());
                    }
                }
                _ => {}
            },
            WalkEvent::Leave(e) => {
                if nested_scope.is_none() {
                    if let Some(expr) = ast::Expr::cast(e.clone()) {
                        match expr {
                            ast::Expr::ReturnExpr(return_expr) => {
                                let expr = return_expr.expr();
                                if let Some(replacement) = make_rewritten_flow(handler, expr) {
                                    ted::replace(return_expr.syntax(), replacement.syntax())
                                }
                            }
                            ast::Expr::BreakExpr(break_expr) if nested_loop.is_none() => {
                                let expr = break_expr.expr();
                                if let Some(replacement) = make_rewritten_flow(handler, expr) {
                                    ted::replace(break_expr.syntax(), replacement.syntax())
                                }
                            }
                            ast::Expr::ContinueExpr(continue_expr) if nested_loop.is_none() => {
                                if let Some(replacement) = make_rewritten_flow(handler, None) {
                                    ted::replace(continue_expr.syntax(), replacement.syntax())
                                }
                            }
                            _ => {
                                // do nothing
                            }
                        }
                    }
                }

                if nested_loop.as_ref() == Some(&e) {
                    nested_loop = None;
                }
                if nested_scope.as_ref() == Some(&e) {
                    nested_scope = None;
                }
            }
        };
    }
}

fn make_rewritten_flow(handler: &FlowHandler, arg_expr: Option<ast::Expr>) -> Option<ast::Expr> {
    let value = match handler {
        FlowHandler::None | FlowHandler::Try { .. } => return None,
        FlowHandler::If { .. } => make::expr_call(
            make::expr_path(make::path_from_text("ControlFlow::Break")),
            make::arg_list(iter::once(make::expr_unit())),
        ),
        FlowHandler::IfOption { .. } => {
            let expr = arg_expr.unwrap_or_else(|| make::expr_tuple(Vec::new()));
            let args = make::arg_list(iter::once(expr));
            make::expr_call(make::expr_path(make::ext::ident_path("Some")), args)
        }
        FlowHandler::MatchOption { .. } => make::expr_path(make::ext::ident_path("None")),
        FlowHandler::MatchResult { .. } => {
            let expr = arg_expr.unwrap_or_else(|| make::expr_tuple(Vec::new()));
            let args = make::arg_list(iter::once(expr));
            make::expr_call(make::expr_path(make::ext::ident_path("Err")), args)
        }
    };
    Some(make::expr_return(Some(value)).clone_for_update())
}