pub mod character_class;
pub mod expression;

use expression::Expression;

use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub struct Rule<'a> {
    name: String,
    expr: Expression<'a>,
}

impl<'a> Rule<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn expr(&self) -> &Expression<'a> {
        &self.expr
    }
}


pub struct Grammar<'a> {
    rules: Vec<Rule<'a>>,
    rulemap: HashMap<String, usize>,
}

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

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    fn add_rule(&mut self, name: &'_ str, expr: Expression<'a>) {
        let rule_index = self.rules.len();
        self.rulemap.insert(name.to_string(), rule_index);
        self.rules.push(Rule { name: name.to_string(), expr });
    }
}


pub enum GrammarBuildError<'a> {
    EmptyRuleSet,
    DuplicateRuleName(&'a str),
}

pub struct GrammarBuilder<'a> {
    grammar: Grammar<'a>,
    called_rules: HashSet<&'a str>,
}

impl<'a, 's> GrammarBuilder<'a> {
    pub fn new() -> GrammarBuilder<'a> {
        GrammarBuilder { grammar: Grammar::new(), called_rules: HashSet::new() }
    }

    pub fn rule(&mut self, name: &'s str, expr: Expression<'a>) -> Result<&mut Self, GrammarBuildError<'s>> {
        if self.grammar.rulemap.contains_key(name) {
            return Err(GrammarBuildError::DuplicateRuleName(name))
        }
        if let Expression::NonTerminal(rule_name) = &expr {
            self.called_rules.insert(rule_name);
        }
        self.grammar.add_rule(name, expr);
        Ok(self)
    }

    pub fn build(self) -> Result<Grammar<'a>, GrammarBuildError<'s>> {
        if self.grammar.is_empty() {
            Err(GrammarBuildError::EmptyRuleSet)
        }
        else {
            Ok(self.grammar)
        }
    }
}

