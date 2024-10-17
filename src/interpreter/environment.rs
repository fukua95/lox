use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::Error, interpreter::Value, token::Token};

#[derive(Clone)]
pub struct Environment {
    cur: HashMap<String, Value>,
    outer_layer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            cur: HashMap::new(),
            outer_layer: None,
        }
    }

    pub fn from(outer: &Rc<RefCell<Environment>>) -> Self {
        Self {
            cur: HashMap::new(),
            outer_layer: Some(Rc::clone(outer)),
        }
    }

    // bind a name to a value.
    pub fn define(&mut self, name: String, value: Value) {
        self.cur.insert(name, value);
    }

    // 处理定义但是没有初始化的变量：
    // Lox 是一门动态类型语言，为了简单，会把变量的值设置为 nil.
    // 所以调用`get`会得到 nil.
    //
    // 处理未定义变量，3种方案:
    // 1. 返回默认值 nil: Lox 不使用这个方案.
    // 2. 返回 syntax error(parse error): 这个最合理，需要考虑:
    //    怎么区分递归函数内定义的变量，比如加前缀等方案.
    // 3. 返回 runtime error, 比较方便，所以 Lox 选择这个方案.
    pub fn get(&self, name: &Token) -> Result<Value, Error> {
        if let Some(v) = self.cur.get(&name.lexeme) {
            return Ok(v.clone());
        }

        if let Some(outer) = &self.outer_layer {
            return outer.borrow().get(name);
        }

        Err(Error::RuntimeError(
            name.clone(),
            format!("Undefined variable '{}'.", &name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Option<Error> {
        if self.cur.contains_key(&name.lexeme) {
            self.cur.insert(name.lexeme, value);
            return None;
        }

        if let Some(outer) = self.outer_layer.as_mut() {
            return outer.borrow_mut().assign(name, value);
        }

        Some(Error::RuntimeError(
            name.clone(),
            format!("Undefined variable '{}'.", &name.lexeme),
        ))
    }
}
