use std::fmt::Error;

use crate::{
    react::{hooks::is_react_component, is_react_call_api, ReactCreateElementCall, ReactLibrary},
    services::semantic::Semantic,
};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, QueryMatch, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsStatement, JsCallExpression,
    JsClassDeclaration, JsFunctionBody, JsIdentifierBinding, JsLanguage, JsMethodClassMember,
    JsObjectExpression, JsPropertyObjectMember, JsReturnStatement, JsStatementList, JsSyntaxKind,
    JsxAttribute, JsxExpressionAttributeValue, TextRange,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, SyntaxNode, SyntaxNodeOptionExt};

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
    type Query = Semantic<FunctionOrClassDeclaration>;
    type State = DiagnosticRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        //let model = ctx.model();

        match node {
            FunctionOrClassDeclaration::AnyJsFunction(function) => {
                if is_function_component(&function) {
                    if is_direct_value_of_render_property_loose(function.syntax().clone()) {
                        return None;
                    }

                    let is_parent_jsx_attribute =
                        JsxAttribute::can_cast(function.syntax().parent()?.kind());
                    let find_parent_jsx_attribute = find_parent_jsx_attribute(function.syntax());
                    let is_inside_jsx_prop_value =
                        is_parent_jsx_attribute || find_parent_jsx_attribute.is_some();
                    if is_inside_jsx_prop_value {
                        if !is_declared_in_render_prop_loose(function) {
                            return Some(DiagnosticRange {
                                inside_component_range: node.syntax().text_range(),
                            });
                        };

                        return None;
                    };

                    // TODO: create react element

                    let parent_function = find_parent_function_node(function.syntax());
                    if parent_function.is_some() {
                        dbg!(&parent_function);

                        let is_render_method = is_direct_value_of_render_property_loose(
                            parent_function?.syntax().parent()?.clone(),
                        );
                        if is_render_method {
                            return None;
                        }
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
    let Some(function_body) = function.body().ok() else {
        return false;
    };

    // JSXを返していたらReact Component
    let is_returning_jsx = match function_body {
        AnyJsFunctionBody::JsFunctionBody(body) => {
            let Some(return_statement_argument) = body.statements().iter().find_map(|statement| {
                let Some(return_statement) = statement.as_js_return_statement() else {
                    return None;
                };
                return_statement.argument().clone()
            }) else {
                return false;
            };

            return_statement_argument.as_jsx_tag_expression().is_some()
        }
        AnyJsFunctionBody::AnyJsExpression(body) => body.as_jsx_tag_expression().is_some(),
    };

    if is_returning_jsx {
        return true;
    }

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
    let parent = find_parent_node(node, is_function_component_node)?;
    Some(AnyJsFunction::cast(parent)?)
}

fn find_parent_class_node(node: &SyntaxNode<JsLanguage>) -> Option<JsClassDeclaration> {
    let parent = find_parent_node(node, is_class_component_node)?;
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

fn compare(node: SyntaxNode<JsLanguage>) -> bool {
    let Some(jsx_expression) = JsxExpressionAttributeValue::cast(node.clone()) else {
        return false;
    };
    let Ok(expression) = jsx_expression.expression() else {
        return false;
    };
    !expression.is_literal_expression()
}

fn find_parent_jsx_attribute(node: &SyntaxNode<JsLanguage>) -> Option<JsxAttribute> {
    let parent = find_parent_node(node, compare)?;
    Some(JsxAttribute::cast(parent.parent()?.parent()?)?)
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

fn is_direct_value_of_render_property_loose(node: SyntaxNode<JsLanguage>) -> bool {
    let Some(method) = JsPropertyObjectMember::cast(node) else {
        return false;
    };
    let Some(method_name) = method.name().ok() else {
        return false;
    };
    method_name.to_trimmed_string().starts_with("render")
}

fn is_declared_in_render_prop_loose(function: &AnyJsFunction) -> bool {
    let Some(parent) = find_parent_jsx_attribute(function.syntax()) else {
        return false;
    };
    let Some(name) = parent.name().ok() else {
        return false;
    };
    name.to_trimmed_string().starts_with("render")
}

fn get_function_id(function: &AnyJsFunction) -> Option<String> {
    match function {
        AnyJsFunction::JsFunctionDeclaration(x) => Some(x.id().ok()?.to_trimmed_string()),
        AnyJsFunction::JsFunctionExportDefaultDeclaration(x) => Some(x.id()?.to_trimmed_string()),
        AnyJsFunction::JsFunctionExpression(x) => Some(x.id()?.to_trimmed_string()),
        AnyJsFunction::JsArrowFunctionExpression(x) => {
            let parent = x.syntax().parent()?;
            if JsPropertyObjectMember::can_cast(parent.kind()) {
                let Some(method_name) = JsPropertyObjectMember::cast(parent)?.name().ok() else {
                    return None;
                };
                Some(method_name.to_trimmed_string())
            } else {
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
}

fn get_class_id(class: &JsClassDeclaration) -> Option<String> {
    Some(class.id().ok()?.to_trimmed_string())
}

// fn is_create_element_call(node: SyntaxNode<JsLanguage>, model: &SemanticModel) -> bool {
//     let Some(node) = JsCallExpression::cast(node) else {
//         return false;
//     };
//     let Some(callee) = node.callee().ok() else {
//         return false;
//     };
//     let callee = callee.omit_parentheses();
//     is_react_call_api(&callee, model, ReactLibrary::ReactDOM, "render")
// }

// fn is_object_expression(node: SyntaxNode<JsLanguage>) -> bool {
//     JsObjectExpression::can_cast(node.kind())
// }

// fn is_inside_create_element_props(model: &SemanticModel, node: &SyntaxNode<JsLanguage>) -> bool {
//     let Some(call) = find_parent_node_with_model(node, model, is_create_element_call) else {
//         return false;
//     };
//     let Some(call) = JsCallExpression::cast(call) else {
//         return false;
//     };
//     let Some(props) = find_parent_node(node, is_object_expression) else {
//         return false;
//     };
//     let Some(props) = JsObjectExpression::cast(props) else {
//         return false;
//     };
//     let Ok(call_args) = call.arguments() else {
//         return false;
//     };
//     let [second_arg] = call_args.get_arguments_by_index([1]);
//     let Some(second_arg) = second_arg else {
//         return false;
//     };

//     props == second_arg
// }

fn find_parent_node<F>(node: &SyntaxNode<JsLanguage>, compare: F) -> Option<SyntaxNode<JsLanguage>>
where
    F: Fn(SyntaxNode<JsLanguage>) -> bool,
{
    let mut parent = node.parent()?;
    while !compare(parent.clone()) {
        parent = parent.parent()?;
    }
    Some(parent)
}
// fn find_parent_node_with_model<F>(
//     node: &SyntaxNode<JsLanguage>,
//     model: &SemanticModel,
//     compare: F,
// ) -> Option<SyntaxNode<JsLanguage>>
// where
//     F: Fn(SyntaxNode<JsLanguage>, &SemanticModel) -> bool,
// {
//     let mut parent = node.parent()?;
//     while !compare(parent.clone(), model) {
//         parent = parent.parent()?;
//     }
//     Some(parent)
// }
