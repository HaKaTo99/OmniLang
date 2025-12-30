use crate::ast;
use serde::{Deserialize, Serialize};

// Mirror runtime guard limits for IR metadata (non-authoritative, informational)
const MAX_LOOP_ITERATIONS: usize = 50;
const MAX_LOOP_TIME_MS: u128 = 1_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyIR {
    pub intent: Option<String>,
    pub actors: Vec<ActorIR>,
    pub context: Option<ContextIR>,
    pub assumptions: Vec<String>,
    pub rules: Vec<RuleIR>,
    pub flat_rules: Vec<RuleIR>,
    pub constraints: Vec<ConstraintIR>,
    pub impacts: Vec<ImpactIR>,
    pub traces: Vec<TraceIR>,
    pub reviews: Vec<ReviewIR>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorIR {
    pub role: String,
    pub primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextIR {
    pub domain: Option<String>,
    pub location: Option<String>,
    pub phase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RuleIR {
    Standard(StandardRuleIR),
    For(ForLoopIR),
    While(WhileLoopIR),
    Match(PolicyMatchRuleIR),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardRuleIR {
    pub condition: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForLoopIR {
    pub iterator: String,
    pub collection: String,
    pub body: Vec<RuleIR>,
    pub guard: GuardMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileLoopIR {
    pub condition: String,
    pub body: Vec<RuleIR>,
    pub guard: GuardMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyMatchRuleIR {
    pub scrutinee: String,
    pub arms: Vec<PolicyMatchArmIR>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyMatchArmIR {
    pub pattern: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardMeta {
    pub max_iterations: usize,
    pub max_time_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintIR {
    pub kind: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactIR {
    pub kind: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceIR {
    pub kind: String,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewIR {
    pub interval: String,
    pub criteria: String,
}

pub fn build_policy_ir(policy: &ast::Policy) -> PolicyIR {
    let rules_tree: Vec<RuleIR> = policy.rules.iter().map(build_rule_ir).collect();
    let mut flat_rules: Vec<RuleIR> = Vec::new();
    for r in &policy.rules {
        flatten_rule_ir(r, &mut flat_rules);
    }

    PolicyIR {
        intent: policy.intent.clone(),
        actors: policy
            .actors
            .iter()
            .map(|a| ActorIR {
                role: a.role.clone(),
                primary: a.primary,
            })
            .collect(),
        context: policy.context.as_ref().map(|c| ContextIR {
            domain: c.domain.clone(),
            location: c.location.clone(),
            phase: c.phase.clone(),
        }),
        assumptions: policy.assumptions.clone(),
        rules: rules_tree,
        flat_rules,
        constraints: policy
            .constraints
            .iter()
            .map(|c| ConstraintIR {
                kind: c.kind.clone(),
                description: c.description.clone(),
            })
            .collect(),
        impacts: policy
            .impacts
            .iter()
            .map(|i| ImpactIR {
                kind: i.kind.clone(),
                description: i.description.clone(),
            })
            .collect(),
        traces: policy
            .traces
            .iter()
            .map(|t| TraceIR {
                kind: t.kind.clone(),
                link: t.link.clone(),
            })
            .collect(),
        reviews: policy
            .reviews
            .iter()
            .map(|r| ReviewIR {
                interval: r.interval.clone(),
                criteria: r.criteria.clone(),
            })
            .collect(),
    }
}

fn build_rule_ir(rule: &ast::Rule) -> RuleIR {
    match rule {
        ast::Rule::Standard(r) => RuleIR::Standard(StandardRuleIR {
            condition: r.condition.clone(),
            action: r.action.clone(),
        }),
        ast::Rule::For(f) => RuleIR::For(ForLoopIR {
            iterator: f.iterator.clone(),
            collection: f.collection.clone(),
            body: f.body.iter().map(build_rule_ir).collect(),
            guard: GuardMeta {
                max_iterations: MAX_LOOP_ITERATIONS,
                max_time_ms: MAX_LOOP_TIME_MS,
            },
        }),
        ast::Rule::While(w) => RuleIR::While(WhileLoopIR {
            condition: w.condition.clone(),
            body: w.body.iter().map(build_rule_ir).collect(),
            guard: GuardMeta {
                max_iterations: MAX_LOOP_ITERATIONS,
                max_time_ms: MAX_LOOP_TIME_MS,
            },
        }),
        ast::Rule::Match(m) => RuleIR::Match(PolicyMatchRuleIR {
            scrutinee: m.scrutinee.clone(),
            arms: m
                .arms
                .iter()
                .map(|a| PolicyMatchArmIR {
                    pattern: a.pattern.clone(),
                    action: a.action.clone(),
                })
                .collect(),
        }),
    }
}

fn flatten_rule_ir(rule: &ast::Rule, out: &mut Vec<RuleIR>) {
    match rule {
        ast::Rule::Standard(_) => out.push(build_rule_ir(rule)),
        ast::Rule::For(f) => {
            let ir = RuleIR::For(ForLoopIR {
                iterator: f.iterator.clone(),
                collection: f.collection.clone(),
                body: Vec::new(), // flattened separately
                guard: GuardMeta {
                    max_iterations: MAX_LOOP_ITERATIONS,
                    max_time_ms: MAX_LOOP_TIME_MS,
                },
            });
            out.push(ir);
            for sub in &f.body {
                flatten_rule_ir(sub, out);
            }
        }
        ast::Rule::While(w) => {
            let ir = RuleIR::While(WhileLoopIR {
                condition: w.condition.clone(),
                body: Vec::new(), // flattened separately
                guard: GuardMeta {
                    max_iterations: MAX_LOOP_ITERATIONS,
                    max_time_ms: MAX_LOOP_TIME_MS,
                },
            });
            out.push(ir);
            for sub in &w.body {
                flatten_rule_ir(sub, out);
            }
        }
        ast::Rule::Match(_) => out.push(build_rule_ir(rule)),
    }
}
