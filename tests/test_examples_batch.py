from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.append(str(ROOT / "src"))

from omnilang import Lexer, Parser, Interpreter  # noqa: E402


SAMPLE_DATA = {
    "SystemStatus": "Ready",
    "Suhu": 52.0,
    "Mode": 1.0,
    "Status": 1.0,
    "Temperature": 55.0,
    "BatteryLevel": 15.0,
    "ObstacleDetected": 1.0,
    "OutsideTemperature": 10.0,
    "RenewableEnergyAvailability": 85.0,
    "CPU_Utilization": 5.0,
    "AverageTemperature": 27.0,
    "CoolingFanSpeed": 50.0,
    "BiasScore": 0.08,
    "TransparencyLevel": 90.0,
    "ProtectedAttributeDetected": 1.0,
    "ProcessingTime": 6.0,
    "QueueLength": 120.0,
    "Applicants": 10.0,
    "AuthenticationGaps": 1,
    "AnomalousBehaviorDetected": 1,
    "TrafficEncryption": "None",
    "AccessDuration": 9,
    "ReAuthenticationFailed": 1,
    "TransactionAmount": 15000,
    "SenderCountry": "US",
    "VolumeIncrease": 400,
    "RiskPatternDetected": 1,
    "InvestigationStatus": "Pending",
    "SupplierRating": 2,
    "ShippingPostponed": 72,
    "HumiditySensor": 80,
    "ShipStatus": "AtPort",
    "CustomsDelay": 6,
    "AnomalyScore": 0.8,
    "Region": "APAC",
    "agent.Status": "Degraded",
    "agent.BufferUsage": 80,
}


class TestExamplesBatch(unittest.TestCase):
    def test_parse_and_eval_all_examples(self):
        examples = sorted((ROOT / "examples").glob("*.omni"))
        failures = []
        for p in examples:
            try:
                code = p.read_text(encoding="utf-8")
                lexer = Lexer(code)
                tokens = lexer.tokenize()
                parser = Parser(tokens)
                policy = parser.parse()
                interp = Interpreter(SAMPLE_DATA.copy())
                interp.evaluate(policy)
            except Exception as exc:  # noqa: BLE001
                failures.append((p.name, str(exc)))
        if failures:
            lines = [f"{name}: {msg}" for name, msg in failures]
            self.fail("\n".join(lines))


if __name__ == "__main__":
    unittest.main()
