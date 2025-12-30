from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.append(str(ROOT / "src"))

from omnilang import Lexer, Parser, Interpreter  # noqa: E402


class TestMatchLambda(unittest.TestCase):
    def test_pattern_match_condition(self):
        code = """
INTENT: Test
ACTOR:
- Primary: Tester
RULE:
- IF match Status { "Ready" => true, _ => false } THEN Display "ok"
"""
        interp = self._run(code, {"Status": "Ready"})
        self.assertTrue(any("ok" in a for a in interp.triggered_actions))

    def test_lambda_hof_reduce(self):
        code = """
INTENT: Test
ACTOR:
- Primary: Tester
RULE:
- IF reduce(map([1, 2, 3], |x| x * 2), |acc, x| acc + x, 0) == 12 THEN Display "hof"
"""
        interp = self._run(code, {})
        self.assertTrue(any("hof" in a for a in interp.triggered_actions))

    def _run(self, code: str, data: dict) -> Interpreter:
        lexer = Lexer(code)
        tokens = lexer.tokenize()
        policy = Parser(tokens).parse()
        interp = Interpreter(data.copy())
        interp.evaluate(policy)
        return interp


if __name__ == "__main__":
    unittest.main()
