"""
OmniLang v1.0 Interpreter (Universal Intent Language)
Author: Herman Krisnanto
Description: Python fallback interpreter for OmniLang v1.0.
Supports: INTENT, ACTOR, CONTEXT, ASSUMPTION, RULE, CONSTRAINT, IMPACT, TRACE, REVIEW.
"""
import re
import sys
import functools
from typing import List, Any, Dict, Optional, Callable

# Token types for v1.0
TOKEN_TYPES = [
    ('COMMENT', r'//.*|#.*'),
    ('INTENT', r'INTENT'),
    ('ACTOR', r'ACTOR'),
    ('CONTEXT', r'CONTEXT'),
    ('ASSUMPTION', r'ASSUMPTION'),
    ('RULE', r'RULE'),
    ('CONSTRAINT', r'CONSTRAINT'),
    ('IMPACT', r'IMPACT'),
    ('TRACE', r'TRACE'),
    ('REVIEW', r'REVIEW'),
    
    ('PRIMARY', r'Primary'),
    ('SECONDARY', r'Secondary'),
    ('DOMAIN', r'Domain'),
    ('LOKASI', r'Lokasi'),
    ('FASE', r'Fase'),
    ('IF', r'IF'),
    ('THEN', r'THEN'),
    ('LEGAL', r'Legal'),
    ('ETHICAL', r'Ethical'),
    ('TECHNICAL', r'Technical'),
    ('BENEFIT', r'Benefit'),
    ('RISK', r'Risk'),
    ('TRADEOFF', r'Trade-off'),
    ('MORAL', r'Moral'),
    ('REGULATION', r'Regulation'),
    ('EVIDENCE', r'Evidence'),

    ('FOR', r'FOR'),
    ('WHILE', r'WHILE'),
    ('IN', r'IN'),

    ('NUMBER', r'\d+(\.\d+)?'),
    ('STRING', r'"[^"]*"'),
    ('IDENT', r'[A-Za-z_][A-Za-z0-9_]*'),
    
    ('LBRACE', r'\{'),
    ('RBRACE', r'\}'),
    ('LBRACKET', r'\['),
    ('RBRACKET', r'\]'),
    ('LPAREN', r'\('),
    ('RPAREN', r'\)'),
    ('PIPE', r'\|'),
    ('STAR', r'\*'),
    ('PLUS', r'\+'),
    ('COLON', r':'),
    ('COMMA', r','),
    ('DOT', r'\.'),
    ('SLASH', r'/'),
    ('MINUS', r'-'),
    ('PERCENT', r'%'),
    ('EQEQ', r'=='),
    ('NEQ', r'!='),
    ('LTE', r'<='),
    ('GTE', r'>='),
    ('LT', r'<'),
    ('GT', r'>'),
    ('EQ', r'='),
    ('NEWLINE', r'\n'),
    ('WS', r'[ \t\r\f\v]+'),
]

class Token:
    def __init__(self, type_, value, line):
        self.type = type_
        self.value = value
        self.line = line
    def __repr__(self):
        return f"Token({self.type}, {self.value}, L{self.line})"

class Lexer:
    def __init__(self, code):
        self.code = code
        self.tokens = []
        self.line = 1
        
    def tokenize(self):
        pos = 0
        while pos < len(self.code):
            match = None
            for type_, regex in TOKEN_TYPES:
                pattern = re.compile(regex, re.IGNORECASE if type_ in [
                    'INTENT', 'ACTOR', 'CONTEXT', 'ASSUMPTION', 'RULE', 
                    'CONSTRAINT', 'IMPACT', 'TRACE', 'REVIEW', 'IF', 'THEN'
                ] else 0)
                match = pattern.match(self.code, pos)
                if match:
                    val = match.group(0)
                    if type_ == 'WS':
                        pass
                    elif type_ == 'NEWLINE':
                        self.line += 1
                        self.tokens.append(Token(type_, val, self.line))
                    elif type_ != 'COMMENT':
                        self.tokens.append(Token(type_, val, self.line))
                    pos = match.end(0)
                    break
            if not match:
                # Handle unknown characters as generic identifiers if possible, or error
                char = self.code[pos]
                if char.isspace():
                    if char == '\n': self.line += 1
                    pos += 1
                    continue
                raise SyntaxError(f"Illegal character '{char}' at line {self.line}")
        
        self.tokens.append(Token('EOF', '', self.line))
        return self.tokens

# AST Nodes
class Policy:
    def __init__(self, intent, actors, context, assumptions, rules, constraints, impacts, traces, reviews):
        self.intent = intent
        self.actors = actors
        self.context = context
        self.assumptions = assumptions
        self.rules = rules
        self.constraints = constraints
        self.impacts = impacts
        self.traces = traces
        self.reviews = reviews

class Actor:
    def __init__(self, role, primary):
        self.role = role
        self.primary = primary

class Context:
    def __init__(self, domain, location, phase):
        self.domain = domain
        self.location = location
        self.phase = phase

class Rule:
    def __init__(self, condition, action):
        self.condition = condition
        self.action = action

class LoopRule:
    def __init__(self, type_, iterator, collection, condition, body):
        self.type = type_ # 'FOR' or 'WHILE'
        self.iterator = iterator
        self.collection = collection
        self.condition = condition
        self.body = body

class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    def parse(self):
        intent = None
        actors = []
        context = None
        assumptions = []
        rules = []
        constraints = []
        impacts = []
        traces = []
        reviews = []

        while not self.is_at_end():
            token = self.peek()
            if token.type == 'INTENT':
                self.advance()
                self.consume('COLON')
                intent = self.parse_text_line()
            elif token.type == 'ACTOR':
                self.advance()
                self.consume('COLON')
                actors = self.parse_actors()
            elif token.type == 'CONTEXT':
                self.advance()
                self.consume('COLON')
                context = self.parse_context()
            elif token.type == 'ASSUMPTION':
                self.advance()
                self.consume('COLON')
                assumptions = self.parse_list()
            elif token.type == 'RULE':
                self.advance()
                self.consume('COLON')
                rules = self.parse_rules()
            elif token.type == 'CONSTRAINT':
                self.advance()
                self.consume('COLON')
                constraints = self.parse_kv_list(['LEGAL', 'ETHICAL', 'TECHNICAL'])
            elif token.type == 'IMPACT':
                self.advance()
                self.consume('COLON')
                impacts = self.parse_kv_list(['BENEFIT', 'RISK', 'TRADEOFF'])
            elif token.type == 'TRACE':
                self.advance()
                self.consume('COLON')
                traces = self.parse_kv_list(['MORAL', 'REGULATION', 'EVIDENCE'])
            elif token.type == 'REVIEW':
                self.advance()
                self.consume('COLON')
                reviews = self.parse_reviews()
            else:
                self.advance()
        
        return Policy(intent, actors, context, assumptions, rules, constraints, impacts, traces, reviews)

    def parse_text_line(self):
        content = []
        while not self.is_at_end() and self.peek().type != 'NEWLINE' and not self.is_header_start():
            content.append(self.advance().value)
        if not self.is_at_end() and self.peek().type == 'NEWLINE':
            self.advance() # consume newline
        return " ".join(content).strip()

    def is_header_start(self):
        t = self.peek()
        if t.type in ['INTENT', 'ACTOR', 'CONTEXT', 'ASSUMPTION', 'RULE', 'CONSTRAINT', 'IMPACT', 'TRACE', 'REVIEW']:
            if self.pos + 1 < len(self.tokens) and self.tokens[self.pos+1].type == 'COLON':
                return True
        return False

    def skip_newlines(self):
        while not self.is_at_end() and self.peek().type == 'NEWLINE':
            self.advance()

    def parse_actors(self):
        actors = []
        self.skip_newlines()
        while self.match('MINUS'):
            is_primary = False
            if self.match('PRIMARY'):
                is_primary = True
            elif self.match('SECONDARY'):
                is_primary = False
            else:
                pass
            self.consume('COLON')
            role = self.parse_text_line()
            actors.append(Actor(role, is_primary))
            self.skip_newlines()
        return actors

    def parse_context(self):
        ctx = {'Domain': None, 'Lokasi': None, 'Fase': None}
        self.skip_newlines()
        while self.match('MINUS'):
            tok = self.peek()
            if tok.type in ['DOMAIN', 'LOKASI', 'FASE', 'IDENT']:
                key = self.advance().value
                self.consume('COLON')
                val = self.parse_text_line()
                ctx[key] = val
            self.skip_newlines()
        return Context(ctx.get('Domain') or ctx.get('DOMAIN'), ctx.get('Lokasi') or ctx.get('LOKASI'), ctx.get('Fase') or ctx.get('FASE'))

    def parse_list(self):
        items = []
        self.skip_newlines()
        while self.match('MINUS'):
            items.append(self.parse_text_line())
            self.skip_newlines()
        return items

    def parse_kv_list(self, valid_keys):
        items = []
        self.skip_newlines()
        while self.match('MINUS'):
            key = "Unknown"
            if self.peek().type in valid_keys or self.peek().type == 'IDENT':
                key = self.advance().value
                self.consume('COLON')
            desc = self.parse_text_line()
            items.append({'kind': key, 'description': desc})
            self.skip_newlines()
        return items

    def parse_rules(self):
        rules = []
        self.skip_newlines()
        while self.match('MINUS'):
            if self.match('IF'):
                cond = []
                while not self.match('THEN') and not self.is_at_end() and self.peek().type != 'NEWLINE':
                    cond.append(self.advance().value)
                action = self.parse_text_line()
                rules.append(Rule(" ".join(cond).strip(), action))
            elif self.match('FOR'):
                it = self.consume('IDENT').value
                self.consume('IN')
                coll = self.consume('IDENT').value
                self.consume('LBRACE')
                self.skip_newlines()
                body = self.parse_rules_in_block()
                self.skip_newlines()
                self.consume('RBRACE')
                rules.append(LoopRule('FOR', it, coll, None, body))
            elif self.match('WHILE'):
                cond = []
                while not self.peek().type == 'LBRACE' and not self.is_at_end() and self.peek().type != 'NEWLINE':
                    cond.append(self.advance().value)
                self.consume('LBRACE')
                self.skip_newlines()
                body = self.parse_rules_in_block()
                self.skip_newlines()
                self.consume('RBRACE')
                rules.append(LoopRule('WHILE', None, None, " ".join(cond).strip(), body))
            self.skip_newlines()
        return rules

    def parse_rules_in_block(self):
        # A bit simpler than parse_rules because we don't expect 'RULE:' header
        # Just look for '-' list items
        return self.parse_rules()

    def parse_reviews(self):
        reviews = []
        while self.match('MINUS'):
            rev = {'interval': None, 'criteria': None}
            # Simplified: look for interval/criteria in the line
            line = self.parse_text_line().lower()
            if 'interval' in line and ':' in line:
                rev['interval'] = line.split(':', 1)[1].strip()
            if 'criteria' in line and ':' in line:
                rev['criteria'] = line.split(':', 1)[1].strip()
            # If not found by split, it might be separate lines in the real thing
            # but our parse_text_line consumes the whole line.
            # v1.0 spec usually has them as sub-items or same line.
            reviews.append(rev)
        return reviews

    def is_at_end(self):
        return self.peek().type == 'EOF'

    def peek(self):
        return self.tokens[self.pos]

    def advance(self):
        if not self.is_at_end():
            self.pos += 1
        return self.tokens[self.pos-1]

    def match(self, type_):
        if self.peek().type == type_:
            self.advance()
            return True
        return False

    def consume(self, type_):
        if self.peek().type == type_:
            return self.advance()
        raise SyntaxError(f"Expected {type_}, got {self.peek().type} at line {self.peek().line}")

# Simple Interpreter
class Interpreter:
    def __init__(self, data=None):
        self.data = data or {}
        self.triggered_actions = []

    def _eval_match(self, expr: str):
        m = re.match(r"\s*match\s+(.*?)\s*\{(.*)\}\s*", expr, re.S)
        if not m:
            raise ValueError("Invalid match expression")

        scrutinee_raw = m.group(1).strip()
        clauses_raw = m.group(2)

        clauses = []
        buf = []
        depth = 0
        in_string = False
        for ch in clauses_raw:
            if ch == '"' and (len(buf) == 0 or buf[-1] != '\\'):
                in_string = not in_string
            if ch == '{' and not in_string:
                depth += 1
            if ch == '}' and not in_string and depth > 0:
                depth -= 1
            if ch == ',' and depth == 0 and not in_string:
                clause = "".join(buf).strip()
                if clause:
                    clauses.append(clause)
                buf = []
            else:
                buf.append(ch)
        tail = "".join(buf).strip()
        if tail:
            clauses.append(tail)

        def eval_plain(e: str):
            return self._eval_expr(e)

        scrutinee = eval_plain(scrutinee_raw)
        default_expr = None

        for clause in clauses:
            if "=>" not in clause:
                continue
            pat_raw, res_raw = clause.split("=>", 1)
            pat = pat_raw.strip()
            res = res_raw.strip()

            if pat == "_":
                default_expr = res
                continue

            pat_val = eval_plain(pat)
            if scrutinee == pat_val:
                return eval_plain(res)

        if default_expr is not None:
            return eval_plain(default_expr)

        raise ValueError("Non-exhaustive match expression")

    def _rewrite_lambdas(self, expr: str) -> str:
        # Convert pipe lambdas like |x| x*2 or |acc, x| acc + x
        pattern = re.compile(r"\|([^|]+)\|\s*([^,)}]+)")

        def repl(m: re.Match) -> str:
            params = m.group(1).strip()
            body = m.group(2).strip()
            return f"(lambda {params}: {body})"

        return pattern.sub(repl, expr)

    def _eval_expr(self, expr: str):
        trimmed = expr.strip()
        normalized = re.sub(r"=\s*>", "=>", trimmed)
        normalized = re.sub(r"\s*=>\s*", "=>", normalized)
        normalized = re.sub(r"\s*,\s*", ", ", normalized)
        normalized = re.sub(r"\s*\{\s*", " { ", normalized)
        normalized = re.sub(r"\s*\}\s*", " } ", normalized)
        normalized = re.sub(r"\s*\(\s*", "(", normalized)
        normalized = re.sub(r"\s*\)\s*", ")", normalized)
        if normalized.lower().startswith("match "):
            return self._eval_match(normalized)

        py_expr = self._rewrite_lambdas(normalized)

        def map_fn(coll, fn: Callable):
            return list(map(fn, coll))

        def filter_fn(coll, fn: Callable):
            return list(filter(fn, coll))

        def reduce_fn(coll, fn: Callable, init=None):
            if init is None:
                return functools.reduce(fn, coll)
            return functools.reduce(fn, coll, init)

        allowed_names: Dict[str, Any] = {
            "map": map_fn,
            "filter": filter_fn,
            "reduce": reduce_fn,
            "sum": sum,
            "len": len,
            "any": any,
            "all": all,
            "True": True,
            "False": False,
            "true": True,
            "false": False,
        }

        for k, v in self.data.items():
            if re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", k):
                allowed_names[k] = v

        return eval(py_expr, {"__builtins__": {}}, allowed_names)

    def evaluate(self, policy):
        print("--- PYTHON RUNTIME EXECUTION (v1.0 Fallback) ---")
        print(f"Intent: {policy.intent}")
        for rule in policy.rules:
            self.eval_rule(rule)
        print("------------------------------------------------")
        return self.triggered_actions

    def eval_rule(self, rule):
        if isinstance(rule, Rule):
            # Skip mocking for richer expressions (match/lambda/HOF)
            cond_lower = rule.condition.lower()
            if not any(k in cond_lower for k in ["match ", "|", "map", "reduce", "filter"]):
                self.apply_mocking(rule.condition)
            
            if self.eval_condition(rule.condition):
                print(f"MATCH -> THEN {rule.action}")
                print(f"-> EXECUTE: {rule.action}") # Studio looks for this
                self.triggered_actions.append(rule.action)
            else:
                pass # print(f"❌ (False): {rule.condition}")
        elif isinstance(rule, LoopRule):
            print(f"  [LOOP] {rule.type} simulation...")
            # Simulate 2 iterations for demo
            for i in range(2):
                for sub in rule.body:
                    self.eval_rule(sub)

    def apply_mocking(self, cond):
        parts = cond.split()
        if not parts: return
        var = parts[0]
        if var not in self.data:
            # Intelligent mocking based on common keywords
            var_lower = var.lower()
            mock_val = 1.0
            if any(k in var_lower for k in ["level", "capacity"]): mock_val = 45.0
            elif any(k in var_lower for k in ["detected", "status", "is"]): mock_val = 1.0
            elif any(k in var_lower for k in ["score", "probability"]): mock_val = 0.75
            elif "distance" in var_lower: mock_val = 10.0
            elif "temp" in var_lower: mock_val = 25.0
            
            print(f"   [MOCK] Variable '{var}' not found. Using simulated value: {mock_val}")
            self.data[var] = mock_val

    def eval_condition(self, cond):
        try:
            return bool(self._eval_expr(cond))
        except Exception:
            # Basic parsing fallback: "Var Op Val"
            parts = cond.split()
            if len(parts) < 3:
                return False
            var = parts[0]
            op = parts[1]
            val_str = parts[2]

            var_val = self.data.get(var, 0.0)
            var_val_str = var_val.strip().strip('"').lower() if isinstance(var_val, str) else None
            var_val_num = None
            if isinstance(var_val, (int, float)):
                var_val_num = float(var_val)
            elif isinstance(var_val, str):
                clean_num = "".join(c for c in var_val if c.isdigit() or c in ['.', '-'])
                if clean_num:
                    try:
                        var_val_num = float(clean_num)
                    except ValueError:
                        var_val_num = None

            val_lower = val_str.lower().strip('"')
            ref_val = None

            if val_lower == "true":
                ref_val = 1.0
            elif val_lower == "false":
                ref_val = 0.0
            else:
                try:
                    clean_val = "".join(c for c in val_str if c.isdigit() or c == '.')
                    if clean_val:
                        ref_val = float(clean_val)
                except Exception:
                    ref_val = None

            if ref_val is not None:
                lhs = var_val_num if var_val_num is not None else var_val
                if op == '<':
                    return lhs < ref_val
                if op == '>':
                    return lhs > ref_val
                if op == '<=':
                    return lhs <= ref_val
                if op == '>=':
                    return lhs >= ref_val
                if op in ['==', '=']:
                    return lhs == ref_val
                if op == '!=':
                    return lhs != ref_val
                if op.lower() == 'in':
                    return True
            else:
                truthy = ["ready", "true", "active", "green", "success"]
                falsy = ["error", "false", "inactive", "red", "failed"]

                if op in ['==', '=']:
                    if val_lower in truthy:
                        return (var_val == 1.0) or (var_val_str in truthy)
                    if val_lower in falsy:
                        return (var_val == 0.0) or (var_val_str in falsy)
                    if var_val_str is not None:
                        return var_val_str == val_lower
                    return str(var_val).lower() == val_lower
                if op == '!=':
                    if val_lower in truthy:
                        return (var_val != 1.0) and (var_val_str not in truthy)
                    if val_lower in falsy:
                        return (var_val != 0.0) and (var_val_str not in falsy)
                    if var_val_str is not None:
                        return var_val_str != val_lower
                    return str(var_val).lower() != val_lower
            return False

def main():
    if len(sys.argv) < 2:
        print("Usage: python omnilang.py <file.omni>")
        return
        
    try:
        with open(sys.argv[1], 'r', encoding='utf-8') as f:
            code = f.read()
            
        lexer = Lexer(code)
        tokens = lexer.tokenize()
        
        parser = Parser(tokens)
        policy = parser.parse()
        
        # Enhanced demo data
        data = {
            "Suhu": 52.0, "Mode": 1.0, "Status": 1.0, "Distance": 0.5,
            "TrafficDensity": 85.0, "EmergencyVehicleDetected": 1.0,
            "EmergencyVehicleDistance": 300.0, "QueueLength": 120.0,
            "DelayInMinutes": 10.0, "BiasScore": 0.08, "TransparencyLevel": 90.0,
            "Applicants": 10.0, "ProcessingTime": 6.0, "ProtectedAttributeDetected": 1.0,
            "WorkerDistance": 0.8, "Temperature": 55.0, "Battery": 15.0,
            "WindSpeed": 30.0, "Altitude": 20.0, "ObstacleDistance": 1.5,
            "ThreatLevel": 90.0, "AnomalyScore": 0.8, "Region": "APAC",
            "agent.Status": "Degraded", "agent.BufferUsage": 80,
        }
        
        itp = Interpreter(data)
        itp.evaluate(policy)
        
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
