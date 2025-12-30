//! Enhanced linter with comprehensive rule set for OmniLang policies

use omnilang_core::ast;
use serde_json::Value;
use std::collections::HashSet;

/// Lint rule severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Individual lint finding
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LintFinding {
    pub rule: String,
    pub severity: Severity,
    pub message: String,
    pub line: Option<usize>,
    pub suggestion: Option<String>,
}

/// Complete lint result
#[derive(Debug)]
pub struct LintResult {
    pub findings: Vec<LintFinding>,
    pub has_errors: bool,
    pub has_warnings: bool,
}

impl LintResult {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            has_errors: false,
            has_warnings: false,
        }
    }

    pub fn add_finding(&mut self, finding: LintFinding) {
        match finding.severity {
            Severity::Error => self.has_errors = true,
            Severity::Warning => self.has_warnings = true,
            _ => {}
        }
        self.findings.push(finding);
    }

    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }
}

/// Main linter implementation
pub struct Linter;

impl Linter {
    pub fn new() -> Self {
        Self
    }

    /// Run all lint rules on a policy
    pub fn lint_policy(&self, policy: &ast::Policy, context: Option<&Value>) -> LintResult {
        let mut result = LintResult::new();

        // Core structure rules
        self.check_required_sections(policy, &mut result);
        self.check_guard_coverage(policy, &mut result);
        self.check_loop_safety(policy, &mut result);
        self.check_unused_sections(policy, &mut result);
        self.check_action_capabilities(policy, &mut result);

        // Context validation
        if let Some(ctx) = context {
            self.check_context_fields(policy, ctx, &mut result);
            self.check_context_usage(policy, ctx, &mut result);
        }

        // Logic validation
        self.check_rule_logic(policy, &mut result);
        self.check_constraint_consistency(policy, &mut result);

        result
    }

    /// Check for required sections in proper order
    fn check_required_sections(&self, policy: &ast::Policy, result: &mut LintResult) {
        // INTENT is required
        if policy.intent.is_none() {
            result.add_finding(LintFinding {
                rule: "required-sections".to_string(),
                severity: Severity::Error,
                message: "Policy must have INTENT section".to_string(),
                line: None,
                suggestion: Some("Add INTENT section at the beginning".to_string()),
            });
        }

        // At least one ACTOR required
        if policy.actors.is_empty() {
            result.add_finding(LintFinding {
                rule: "required-sections".to_string(),
                severity: Severity::Error,
                message: "Policy must have at least one ACTOR".to_string(),
                line: None,
                suggestion: Some("Add ACTOR section with primary and secondary roles".to_string()),
            });
        }

        // At least one RULE required
        if policy.rules.is_empty() {
            result.add_finding(LintFinding {
                rule: "required-sections".to_string(),
                severity: Severity::Warning,
                message: "Policy should have at least one RULE".to_string(),
                line: None,
                suggestion: Some("Add RULE section with IF/THEN logic".to_string()),
            });
        }
    }

    /// Check that loops have proper guard coverage
    fn check_guard_coverage(&self, policy: &ast::Policy, result: &mut LintResult) {
        for rule in &policy.rules {
            if let ast::Rule::For(_) | ast::Rule::While(_) | ast::Rule::Match(_) = rule {
                result.add_finding(LintFinding {
                    rule: "guard-coverage".to_string(),
                    severity: Severity::Info,
                    message: "Complex rule detected - ensure guard limits or logic are optimal".to_string(),
                    line: None,
                    suggestion: Some("Configure appropriate safety guards for this rule type".to_string()),
                });
            }
        }
    }

    /// Check loop safety (caps, infinite loop prevention)
    fn check_loop_safety(&self, policy: &ast::Policy, result: &mut LintResult) {
        for rule in &policy.rules {
            match rule {
                ast::Rule::While(while_loop) => {
                    if !self.is_terminating_condition(&while_loop.condition) {
                        result.add_finding(LintFinding {
                            rule: "loop-safety".to_string(),
                            severity: Severity::Warning,
                            message: format!("WHILE loop condition '{}' may not terminate", while_loop.condition),
                            line: None,
                            suggestion: Some("Add counter or time-based termination condition".to_string()),
                        });
                    }
                }
                ast::Rule::For(for_loop) => {
                    if for_loop.collection.contains("large") || for_loop.collection.contains("all") {
                        result.add_finding(LintFinding {
                            rule: "loop-safety".to_string(),
                            severity: Severity::Warning,
                            message: format!("FOR loop over '{}' may process large collections", for_loop.collection),
                            line: None,
                            suggestion: Some("Consider limiting collection size or adding early termination".to_string()),
                        });
                    }
                }
                ast::Rule::Match(_) | ast::Rule::Standard(_) => {}
            }
        }
    }

    /// Check for unused sections
    fn check_unused_sections(&self, policy: &ast::Policy, result: &mut LintResult) {
        // Check for empty sections that should be removed or populated
        if policy.assumptions.is_empty() {
            result.add_finding(LintFinding {
                rule: "unused-sections".to_string(),
                severity: Severity::Info,
                message: "ASSUMPTION section is empty".to_string(),
                line: None,
                suggestion: Some("Add environmental assumptions or remove section".to_string()),
            });
        }

        if policy.constraints.is_empty() {
            result.add_finding(LintFinding {
                rule: "unused-sections".to_string(),
                severity: Severity::Info,
                message: "CONSTRAINT section is empty".to_string(),
                line: None,
                suggestion: Some("Add legal, ethical, or technical constraints".to_string()),
            });
        }
    }

    /// Check action capabilities alignment
    fn check_action_capabilities(&self, policy: &ast::Policy, result: &mut LintResult) {
        // Extract actions from rules
        let mut actions = HashSet::new();
        for rule in &policy.rules {
            match rule {
                ast::Rule::Standard(std_rule) => {
                    actions.insert(std_rule.action.clone());
                }
                ast::Rule::For(for_rule) => {
                    for sub_rule in &for_rule.body {
                        if let ast::Rule::Standard(std_rule) = sub_rule {
                            actions.insert(std_rule.action.clone());
                        }
                    }
                }
                ast::Rule::While(while_rule) => {
                    for sub_rule in &while_rule.body {
                        if let ast::Rule::Standard(std_rule) = sub_rule {
                            actions.insert(std_rule.action.clone());
                        }
                    }
                }
                ast::Rule::Match(match_rule) => {
                    for arm in &match_rule.arms {
                        actions.insert(arm.action.clone());
                    }
                }
            }
        }

        // Check for actions without clear capabilities
        for action in actions {
            if action.contains("Network") || action.contains("network") {
                result.add_finding(LintFinding {
                    rule: "action-capabilities".to_string(),
                    severity: Severity::Warning,
                    message: format!("Action '{}' requires Network capability", action),
                    line: None,
                    suggestion: Some("Ensure Network capability is granted to executing actor".to_string()),
                });
            }
            if action.contains("File") || action.contains("file") {
                result.add_finding(LintFinding {
                    rule: "action-capabilities".to_string(),
                    severity: Severity::Warning,
                    message: format!("Action '{}' requires FileSystem capability", action),
                    line: None,
                    suggestion: Some("Ensure FileSystem capability is granted to executing actor".to_string()),
                });
            }
        }
    }

    /// Check context field usage and validation
    fn check_context_fields(&self, policy: &ast::Policy, context: &Value, result: &mut LintResult) {
        // Extract referenced fields from rules
        let mut referenced_fields = HashSet::new();
        for rule in &policy.rules {
            match rule {
                ast::Rule::Standard(std_rule) => {
                    self.extract_fields_from_condition(&std_rule.condition, &mut referenced_fields);
                }
                ast::Rule::For(for_rule) => {
                    self.extract_fields_from_condition(&for_rule.collection, &mut referenced_fields);
                    for sub_rule in &for_rule.body {
                        if let ast::Rule::Standard(std_rule) = sub_rule {
                            self.extract_fields_from_condition(&std_rule.condition, &mut referenced_fields);
                        }
                    }
                }
                ast::Rule::While(while_rule) => {
                    self.extract_fields_from_condition(&while_rule.condition, &mut referenced_fields);
                    for sub_rule in &while_rule.body {
                        if let ast::Rule::Standard(std_rule) = sub_rule {
                            self.extract_fields_from_condition(&std_rule.condition, &mut referenced_fields);
                        }
                    }
                }
                ast::Rule::Match(match_rule) => {
                    referenced_fields.insert(match_rule.scrutinee.clone());
                    for arm in &match_rule.arms {
                        referenced_fields.insert(arm.pattern.clone());
                    }
                }
            }
        }

        // Check if referenced fields exist in context
        for field in referenced_fields {
            if !self.context_has_field(context, &field) {
                result.add_finding(LintFinding {
                    rule: "context-validation".to_string(),
                    severity: Severity::Error,
                    message: format!("Context field '{}' is referenced but not provided", field),
                    line: None,
                    suggestion: Some(format!("Add '{}' to context JSON", field)),
                });
            }
        }
    }

    /// Check context field usage patterns
    fn check_context_usage(&self, policy: &ast::Policy, context: &Value, result: &mut LintResult) {
        // Check for unused context fields
        let mut provided_fields = HashSet::new();
        self.collect_context_fields(context, "", &mut provided_fields);

        let mut used_fields = HashSet::new();
        for rule in &policy.rules {
            match rule {
                ast::Rule::Standard(std_rule) => {
                    self.extract_fields_from_condition(&std_rule.condition, &mut used_fields);
                }
                ast::Rule::For(for_rule) => {
                    self.extract_fields_from_condition(&for_rule.collection, &mut used_fields);
                }
                ast::Rule::While(while_rule) => {
                    self.extract_fields_from_condition(&while_rule.condition, &mut used_fields);
                }
                ast::Rule::Match(match_rule) => {
                    used_fields.insert(match_rule.scrutinee.clone());
                }
            }
        }

        for field in provided_fields {
            if !used_fields.contains(&field) {
                result.add_finding(LintFinding {
                    rule: "context-usage".to_string(),
                    severity: Severity::Info,
                    message: format!("Context field '{}' is provided but never used", field),
                    line: None,
                    suggestion: Some("Remove unused field or add rule that uses it".to_string()),
                });
            }
        }
    }

    /// Check rule logic consistency
    fn check_rule_logic(&self, policy: &ast::Policy, result: &mut LintResult) {
        for rule in &policy.rules {
            if let ast::Rule::Standard(std_rule) = rule {
                if std_rule.condition.contains("true") && std_rule.condition.contains("==") {
                    result.add_finding(LintFinding {
                        rule: "rule-logic".to_string(),
                        severity: Severity::Warning,
                        message: "Rule condition may be always true".to_string(),
                        line: None,
                        suggestion: Some("Review condition logic for correctness".to_string()),
                    });
                }
            }
        }
    }

    /// Check constraint consistency
    fn check_constraint_consistency(&self, policy: &ast::Policy, result: &mut LintResult) {
        let mut has_legal = false;
        let mut has_technical = false;
        let mut has_ethical = false;

        for constraint in &policy.constraints {
            match constraint.kind.to_lowercase().as_str() {
                "legal" => has_legal = true,
                "technical" => has_technical = true,
                "ethical" | "ethics" => has_ethical = true,
                _ => {
                    result.add_finding(LintFinding {
                        rule: "constraint-consistency".to_string(),
                        severity: Severity::Warning,
                        message: format!("Unknown constraint kind: '{}'", constraint.kind),
                        line: None,
                        suggestion: Some("Use 'Legal', 'Technical', or 'Ethical'".to_string()),
                    });
                }
            }
        }

        // Suggest comprehensive constraints
        if !has_legal {
            result.add_finding(LintFinding {
                rule: "constraint-consistency".to_string(),
                severity: Severity::Info,
                message: "Consider adding Legal constraints".to_string(),
                line: None,
                suggestion: Some("Add CONSTRAINT with kind 'Legal'".to_string()),
            });
        }

        if !has_technical {
            result.add_finding(LintFinding {
                rule: "constraint-consistency".to_string(),
                severity: Severity::Info,
                message: "Consider adding Technical constraints".to_string(),
                line: None,
                suggestion: Some("Add CONSTRAINT with kind 'Technical'".to_string()),
            });
        }

        if !has_ethical {
            result.add_finding(LintFinding {
                rule: "constraint-consistency".to_string(),
                severity: Severity::Info,
                message: "Consider adding Ethical constraints".to_string(),
                line: None,
                suggestion: Some("Add CONSTRAINT with kind 'Ethical'".to_string()),
            });
        }
    }

    // Helper methods
    fn is_terminating_condition(&self, condition: &str) -> bool {
        // Simple heuristic: conditions with counters or time limits are likely terminating
        condition.contains("count") ||
        condition.contains("time") ||
        condition.contains("limit") ||
        condition.contains("<") ||
        condition.contains(">")
    }

    fn extract_fields_from_condition(&self, condition: &str, fields: &mut HashSet<String>) {
        // Simple field extraction - in real implementation, use proper AST analysis
        for part in condition.split_whitespace() {
            if part.contains('.') && !part.contains('\"') && !part.contains('\'') {
                // Likely a field reference like "sensor.value" or "context.field"
                let field = part.trim_matches(|c: char| !c.is_alphanumeric() && c != '.');
                if !field.is_empty() && field.contains('.') {
                    fields.insert(field.to_string());
                }
            }
        }
    }

    fn context_has_field(&self, context: &Value, field_path: &str) -> bool {
        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = context;

        for part in parts {
            match current.get(part) {
                Some(value) => current = value,
                None => return false,
            }
        }
        true
    }

    fn collect_context_fields(&self, value: &Value, prefix: &str, fields: &mut HashSet<String>) {
        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    fields.insert(new_prefix.clone());
                    self.collect_context_fields(val, &new_prefix, fields);
                }
            }
            Value::Array(arr) => {
                for (i, val) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, i);
                    self.collect_context_fields(val, &new_prefix, fields);
                }
            }
            _ => {}
        }
    }
}

/// Convenience function for linting with optional context
#[allow(dead_code)]
pub fn lint_policy(policy: &ast::Policy, context: Option<&Value>) -> LintResult {
    let linter = Linter::new();
    linter.lint_policy(policy, context)
}

/// Format lint results for display
#[allow(dead_code)]
pub fn format_lint_results(result: &LintResult) -> String {
    let mut output = String::new();

    if result.is_clean() {
        output.push_str("✅ Lint clean: No issues found\n");
        return output;
    }

    for finding in &result.findings {
        let severity_icon = match finding.severity {
            Severity::Error => "❌",
            Severity::Warning => "⚠️",
            Severity::Info => "ℹ️",
        };

        output.push_str(&format!("{} {}: {}\n", severity_icon, finding.rule, finding.message));

        if let Some(suggestion) = &finding.suggestion {
            output.push_str(&format!("   💡 {}\n", suggestion));
        }

        output.push('\n');
    }

    let error_count = result.findings.iter().filter(|f| f.severity == Severity::Error).count();
    let warning_count = result.findings.iter().filter(|f| f.severity == Severity::Warning).count();
    let info_count = result.findings.iter().filter(|f| f.severity == Severity::Info).count();

    output.push_str(&format!("Summary: {} errors, {} warnings, {} info\n",
        error_count, warning_count, info_count));

    output
}
