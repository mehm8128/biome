use std::fmt::Error;

use crate::react::hooks::is_react_component;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, QueryMatch, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsFunction, AnyJsStatement, JsClassDeclaration, JsIdentifierBinding, JsLanguage,
    JsMethodClassMember, JsStatementList, JsSyntaxKind, TextRange,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxNode, SyntaxNodeOptionExt};

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoNestedComponentDefinitions {
        version: "next",
        name: "noNestedComponentDefinitions",
        language: "jsx",
        sources: &[RuleSource::EslintReact("no-nested-components")],
        recommended: false,
    }
}

declare_node_union! {
    pub FunctionOrClassDeclaration = AnyJsFunction | JsClassDeclaration
}

pub struct DiagnosticRange {
    inside_component_range: TextRange,
}

impl Rule for NoNestedComponentDefinitions {
    type Query = Ast<FunctionOrClassDeclaration>;
    type State = DiagnosticRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            FunctionOrClassDeclaration::AnyJsFunction(function) => {
                if is_function_component(&function) {
                    let parent_function = find_parent_function_node(function.syntax());
                    if parent_function.is_some() {
                        dbg!(parent_function);
                        return Some(DiagnosticRange {
                            inside_component_range: node.syntax().text_range(),
                        });
                    }

                    let parent_class_render_method =
                        find_parent_class_render_method(function.syntax());
                    if parent_class_render_method.is_some() {
                        dbg!(parent_class_render_method);
                        return Some(DiagnosticRange {
                            inside_component_range: node.syntax().text_range(),
                        });
                    }
                }

                None
            }
            FunctionOrClassDeclaration::JsClassDeclaration(class) => {
                if is_class_component(&class) {
                    let parent_function = find_parent_function_node(class.syntax());
                    if parent_function.is_some() {
                        dbg!(&parent_function);
                        return Some(DiagnosticRange {
                            inside_component_range: node.syntax().text_range(),
                        });
                    }

                    let parent_class = find_parent_class_node(class.syntax());
                    if parent_class.is_some() {
                        dbg!(&parent_class);
                        return Some(DiagnosticRange {
                            inside_component_range: node.syntax().text_range(),
                        });
                    }
                }

                None
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.inside_component_range,
                markup! {
                    "Do not nest components inside other components. Move it to the top level."
                },
            )
            .note(markup! {
                "TODO"
            }),
        )
    }
}

fn is_function_component(function: &AnyJsFunction) -> bool {
    let Some(id) = get_function_id(function) else {
        return false;
    };
    is_react_component(&id)
}

fn is_class_component(class: &JsClassDeclaration) -> bool {
    let Some(id) = get_class_id(class) else {
        return false;
    };
    is_react_component(&id)
}

fn is_function_component_node(node: SyntaxNode<JsLanguage>) -> bool {
    let Some(function) = AnyJsFunction::cast(node) else {
        return false;
    };
    is_function_component(&function)
}

fn is_class_component_node(node: SyntaxNode<JsLanguage>) -> bool {
    let Some(function) = JsClassDeclaration::cast(node) else {
        return false;
    };
    is_class_component(&function)
}

fn find_parent_function_node(node: &SyntaxNode<JsLanguage>) -> Option<AnyJsFunction> {
    let mut parent = node.parent()?;
    while !is_function_component_node(parent.clone()) {
        parent = parent.parent()?;
    }
    Some(AnyJsFunction::cast(parent)?)
}

fn find_parent_class_node(node: &SyntaxNode<JsLanguage>) -> Option<JsClassDeclaration> {
    let mut parent = node.parent()?;
    while !is_class_component_node(parent.clone()) {
        parent = parent.parent()?;
    }
    Some(JsClassDeclaration::cast(parent)?)
}

fn find_parent_class_render_method(node: &SyntaxNode<JsLanguage>) -> Option<JsMethodClassMember> {
    let mut parent = node.parent()?;
    while !(is_class_component_node(parent.parent()?.parent()?.clone())
        && is_render_method(parent.clone()))
    {
        parent = parent.parent()?;
    }

    Some(JsMethodClassMember::cast(parent)?)
}

fn is_render_method(node: SyntaxNode<JsLanguage>) -> bool {
    let Some(method) = JsMethodClassMember::cast(node) else {
        return false;
    };
    let Some(method_name) = method.name().ok() else {
        return false;
    };
    method_name.to_trimmed_string() == "render"
}

fn get_function_id(function: &AnyJsFunction) -> Option<String> {
    match function {
        AnyJsFunction::JsFunctionDeclaration(x) => Some(x.id().ok()?.to_trimmed_string()),
        AnyJsFunction::JsFunctionExportDefaultDeclaration(x) => Some(x.id()?.to_trimmed_string()),
        AnyJsFunction::JsFunctionExpression(x) => Some(x.id()?.to_trimmed_string()),
        AnyJsFunction::JsArrowFunctionExpression(x) => {
            let id_node = x.syntax().parent()?.parent()?.first_child()?;
            Some(
                JsIdentifierBinding::cast(id_node)?
                    .name_token()
                    .ok()?
                    .to_string(),
            )
        }
    }
}

fn get_class_id(class: &JsClassDeclaration) -> Option<String> {
    Some(class.id().ok()?.to_trimmed_string())
}
