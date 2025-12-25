
// src/ast.rs

// Root dari dokumen Kebijakan OmniLang
#[derive(Debug)]
pub struct Policy {
    pub intent: Option<String>,
    pub actors: Vec<Actor>,
    pub context: Option<Context>,
    pub rules: Vec<Rule>,
    pub constraints: Vec<Constraint>,
    pub impacts: Vec<Impact>,
    pub traces: Vec<Trace>,
}

#[derive(Debug)]
pub struct Actor {
    pub role: String,
    pub primary: bool,
}

#[derive(Debug)]
pub struct Context {
    pub domain: Option<String>,
    pub location: Option<String>,
    pub phase: Option<String>,
}

#[derive(Debug)]
pub struct Rule {
    pub condition: String,
    pub action: String,
}

#[derive(Debug)]
pub struct Constraint {
    pub kind: String, // Legal, Ethical, Technical
    pub description: String,
}

#[derive(Debug)]
pub struct Impact {
    pub kind: String, // Benefit, Risk
    pub description: String,
}

#[derive(Debug)]
pub struct Trace {
    pub kind: String, // Moral, Regulation, Evidence
    pub link: String,
}

