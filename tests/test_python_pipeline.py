import sys
from pathlib import Path
import unittest

# Ensure src/ is on import path for the Python prototype
ROOT = Path(__file__).resolve().parents[1]
sys.path.append(str(ROOT / "src"))

from omnilang import Lexer, Parser, Interpreter

class TestPythonPipelineV1(unittest.TestCase):
    def test_basic_policy_parsing(self):
        code = """
INTENT: Test intent
ACTOR:
- Primary: Tester
CONTEXT:
- Domain: Testing
RULE:
- IF Signal > 90 THEN Alert
CONSTRAINT:
- Technical: SystemUp
IMPACT:
- Benefit: High uptime
TRACE:
- Evidence: Logs
"""
        lexer = Lexer(code)
        tokens = lexer.tokenize()
        parser = Parser(tokens)
        policy = parser.parse()
        
        self.assertEqual(policy.intent, "Test intent")
        self.assertEqual(len(policy.actors), 1)
        self.assertEqual(policy.actors[0].role, "Tester")
        self.assertEqual(len(policy.rules), 1)
        self.assertEqual(policy.rules[0].condition, "Signal > 90")

    def test_interpreter_execution(self):
        code = """
RULE:
- IF Temperature > 50 THEN ActivateCooling
"""
        lexer = Lexer(code)
        tokens = lexer.tokenize()
        parser = Parser(tokens)
        policy = parser.parse()
        
        # Test with condition met
        data = {"Temperature": 55.0}
        interp = Interpreter(data)
        actions = interp.evaluate(policy)
        self.assertIn("ActivateCooling", actions)
        
        # Test with condition not met
        data_safe = {"Temperature": 40.0}
        interp_safe = Interpreter(data_safe)
        actions_safe = interp_safe.evaluate(policy)
        self.assertNotIn("ActivateCooling", actions_safe)

if __name__ == "__main__":
    unittest.main()
