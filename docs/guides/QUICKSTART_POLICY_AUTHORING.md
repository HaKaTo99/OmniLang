# 🚀 Quickstart Guide: Menulis Kebijakan OmniLang

Panduan 10 langkah untuk membuat kebijakan autonomous systems dengan OmniLang.

## 📋 Prerequisites
- Basic understanding of policy logic
- JSON context data structure
- Domain knowledge (IoT, AI, autonomous systems)

## 🏗️ Step 1: Define Intent
```omni
INTENT: [Clear, actionable goal for your policy]
```
**Example:** `INTENT: Smart City Traffic Optimization`

## 👥 Step 2: Identify Actors
```omni
ACTOR:
- Primary: [Main decision maker]
- Secondary: [Supporting systems/users]
```
**Example:**
```omni
ACTOR:
- Primary: Traffic Control System
- Secondary: Emergency Services
```

## 📊 Step 3: Structure Context
```json
{
  "domain": "your_domain",
  "key_metric1": value,
  "key_metric2": value
}
```
**Example:**
```json
{
  "domain": "smart_city",
  "traffic_density": 85,
  "emergency_vehicle": true,
  "weather_condition": "rain"
}
```

## ⚖️ Step 4: Write Basic Rules
```omni
RULE:
- IF [condition]
- THEN [action]
```
**Example:**
```omni
RULE:
- IF TrafficDensity > 80 AND EmergencyVehicle == true
- THEN GreenLightPriority, LogEmergency
```

## 🔄 Step 5: Add Loops for Collections
```omni
RULE:
- FOR item IN Collection
- IF [condition on item]
- THEN [action]
```
**Example:**
```omni
RULE:
- FOR sensor IN TrafficSensors
- IF sensor.Status != "operational"
- THEN MaintenanceAlert, DeactivateSensor
```

## 🧮 Step 6: Use Stdlib Functions
```omni
RULE:
- IF std::math::avg(SensorReadings) > Threshold
- THEN AlertMaintenance
```
**Available stdlib:**
- `std::math::*` (clamp, round, avg)
- `std::string::*` (contains, split, format)
- `std::time::*` (now, diff, format)

## ⏱️ Step 7: Add Time-Based Rules
```omni
RULE:
- IF std::time::duration_between_ms(LastCheck, std::time::now_unix_millis()) > 3600000
- THEN PeriodicMaintenance
```

## 📈 Step 8: Define Impact Metrics
```omni
IMPACT:
- MetricName: value
- AnotherMetric: value
```
**Example:**
```omni
IMPACT:
- Efficiency: 92.5%
- SafetyIncidents: 0
- ResponseTime: 2.3
```

## 🔍 Step 9: Add Trace Information
```omni
TRACE:
- PolicyVersion: "v1.0"
- LastUpdated: "2024-01-20"
- Standards: "ISO 9001"
```

## ✅ Step 10: Test & Validate
```bash
# Lint your policy
omnilang lint your_policy.omni

# Test with context
omnilang test your_policy.omni --context context.json

# Get metrics
omnilang metrics your_policy.omni --format prometheus
## 🤖 Step 11: Distributed AI Intelligence
```omni
// Menggunakan AI dari node lain
@mesh(target: "127.0.0.1:8081")
@oracle(model: "traffic_optimizer.onnx")
fn optimizeFlow(input: [f64]) -> [f64];

RULE:
- IF TrafficDensity > 90
- THEN optimizeFlow(LiveData)
```
OmniLang memungkinkan kebijakan Anda memanggil kekuatan AI lintas jaringan secara transparan.

## 🎯 Common Patterns

### Safety-First Pattern
```omni
RULE:
- IF SafetyCondition == false
- THEN EmergencyStop, AlertAll

RULE:
- IF NormalConditions == true
- THEN ResumeOperations
```

### Threshold Monitoring
```omni
RULE:
- IF Metric > UpperThreshold
- THEN ReduceLoad, ScaleUp

RULE:
- IF Metric < LowerThreshold
- THEN OptimizeUsage, ScaleDown
```

### Time-Based Maintenance
```omni
RULE:
- IF std::time::duration_between_ms(LastMaintenance, std::time::now_unix_millis()) > 604800000
- THEN ScheduleMaintenance, LogDue
```

## 🐛 Troubleshooting

### Common Issues:
1. **Context field not found** → Check JSON structure
2. **Type mismatch** → Use correct data types
3. **Loop without guard** → Add safety limits
4. **Action not defined** → Implement action handlers

### Debug Commands:
```bash
# Verbose parsing
omnilang exec policy.omni --verbose

# Check context validation
omnilang test policy.omni --context context.json
```

## 📚 Next Steps
- Read domain-specific examples
- Learn advanced patterns in SPEC_V1.0.md
- Join community discussions

**Happy policy authoring! 🎉**
