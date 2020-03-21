pub mod character_class;
pub mod expression;

use expression::Expression;

use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub struct Rule<'a> {
    name: String,
    expr: Expression<'a>,
}

pub struct Grammar<'a> {
    rules: Vec<Rule<'a>>,
    rulemap: HashMap<String, usize>,
}

pub struct EmptyRuleSet;

impl<'a> Grammar<'a> {
    pub fn new() -> Grammar<'a> {
        Grammar { rules: Vec::new(), rulemap: HashMap::new() }
    }

    pub fn rule(&self, name: &str) -> Option<&Rule> {
        self.rulemap.get(name).and_then(|&index| { Some(&self.rules[index]) })
    }

    pub fn axiom(&self) -> Option<&Rule> {
        self.rules.first()
    }

    pub fn add_rule(&mut self, name: &'_ str, expr: Expression<'a>) {
        let len = self.rules.len();
        self.rules.push(Rule { name: name.to_string(), expr });
        self.rulemap.insert(name.to_string(), len);
    }
}


pub struct GrammarBuilder<'a> {
    grammar: Grammar<'a>,
    called_rules: HashSet<String>,
}

impl<'a> GrammarBuilder<'a> {
    pub fn new() -> GrammarBuilder<'a> {
        GrammarBuilder { grammar: Grammar::new(), called_rules: HashSet::new() }
    }

    pub fn rule(&mut self, name: &'_ str, expr: Expression<'a>) -> &mut Self {
        if let Expression::NonTerminal(rule_name) = &expr {
            self.called_rules.insert(rule_name.to_string());
        }
        self.grammar.add_rule(name, expr);
        self
    }

    pub fn build(self) -> Grammar<'a> {
        self.grammar
    }
}

