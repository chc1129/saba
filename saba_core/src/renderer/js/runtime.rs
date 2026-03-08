
use crate::renderer::dom::node::Node as DomNode;

use crate::renderer::js::ast::Node;
use crate::renderer::js::ast::Program;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;

use core::borrow::Borrow;
use core::cell::RefCell;


use core::ops::Add;
use core::ops::Sub;



/// https://262.ecma-international.org/#sec-ecmascript-language-types
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    /// https://262.ecma-international.org/#sec-numeric-types
    Number(u64),
}







impl Add<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn add(self, rhs: RuntimeValue) -> RuntimeValue {
        if let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs) {
            return RuntimeValue::Number(left_num + right_num);
        }

        RuntimeValue::StringLiteral(self.to_string() + &rhs.to_string())
    }
}

impl Sub<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn sub(self, rhs: RuntimeValue) -> RuntimeValue {
        if let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs) {
            return RuntimeValue::Number(left_num - right_num);
        }

        // NaN: Not a Number
        RuntimeValue::Number(u64::MIN)
    }
}








/// https://262.ecma-international.org/#sec-environment-records
#[derive(Debug, Clone)]
pub struct Environment {
    variables: VariableMap,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            variables: VariableMap::new(),
            outer,
        }
    }

    pub fn get_variable(&self, name: String) -> Option<RuntimeValue> {
        for variable in &self.variables {
            if variable.0 == name {
                return variable.1.clone(); // (d1)
            }
        }
        if let Some(env) = &self.outer {
            env.borrow_mut().get_variable(name) // (d2)
        } else {
            None
        }
    }

    fn add_variable(&mut self, name: String, value: Option<RuntimeValue>) {
        self.variables.push((name, value));
    }

    fn update_variable(&mut self, name: String, value: Option<RuntimeValue>) {
        for i in 0..self.variables.len() {
            // もし変数を見つけた場合、今までの名前と値のペアを削除し、新しい値とのペアを追加する
            if self.variables[i].0 == name {
                self.variables.remove(i);
                self.variables.push((name, value));
                return;
            }
        }
    }
}




#[derive(Debug, Clone)]
pub struct JsRuntime {
    dom_root: Rc<RefCell<DomNode>>,
    env: Rc<RefCell<Environment>>,
    functions: Vec<Function>,
}

impl JsRuntime {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new(None))),
        }
    }



    fn eval(
        &mut self,
        node: &Option<Rc<Node>>,
    ) -> Option<RuntimeValue> {
        let node = match node {
            Some(n) => n,
            None => return None,
        };

        match node.borrow() {
            Node::ExpressionStatement(expr) => return self.eval(&expr, env.clone()),
            Node::AdditiveExpression {
                operator,
                left,
                right,
            } => {
                let left_value = match self.eval(&left, env.clone()) {
                    Some(value) => value,
                    None => return None,
                };
                let right_value = match self.eval(&right, env.clone()) {
                    Some(value) => value,
                    None => return None,
                };

                if operator == &'+' {
                    Some(left_value + right_value)
                } else if operator == &'-' {
                    Some(left_value - right_value)
                } else {
                    None
                }
            }
            Node::AssignmentExpression {
                operator,
                left,
                right,
            } => {
                // 後ほど実装
                 None
            }
            Node::NumericLiteral(value) => Some(RuntimeValue::Number(*value)),
            // ノードの種類を追加したので、デフォルトアームを追加
            // 後ほど削除する
            _ => todo!(),
        }
    }











    pub fn execute(&mut self, program: &Program) {
        for node in program.body() {
            self.eval(&Some(node.clone()), self.env.clone());
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::js::ast::JsParser;
    use crate::renderer::js::token::JsLexer;

    #[test]
    fn test_num() {
        let dom = Rc::new(RefCell::new(DomNode::new(DomNodeKind::Document)));
        let input = "42".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new(dom);
        let expected = [Some(RuntimeValue::Number(42))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()), runtime.env.clone());
            assert_eq!(expected[i], result);
            i += 1;
        }
    }

    #[test]
    fn test_add_nums() {
        let dom = Rc::new(RefCell::new(DomNode::new(DomNodeKind::Document)));
        let input = "1 + 2".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new(dom);
        let expected = [Some(RuntimeValue::Number(3))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()), runtime.env.clone());
            assert_eq!(expected[i], result);
            i += 1;
        }
    }

    #[test]
    fn test_sub_nums() {
        let dom = Rc::new(RefCell::new(DomNode::new(DomNodeKind::Document)));
        let input = "2 - 1".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new(dom);
        let expected = [Some(RuntimeValue::Number(1))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()), runtime.env.clone());
            assert_eq!(expected[i], result);
            i += 1;
        }
    }




}
