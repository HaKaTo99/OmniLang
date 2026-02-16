"use client";

import { useEffect, useRef, useState } from "react";
import { useSearchParams } from "next/navigation";
import { Header } from "@/components/header";
import { Documentation } from "@/components/documentation";
import { CodeEditorPanel } from "@/components/code-editor-panel";
import { useToast } from "@/hooks/use-toast";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { CUIPanel } from "@/components/cui-panel";
import { VUIButton, speak } from "@/components/vui-button";
import { NUIWrapper, FileDropZone } from "@/components/nui-wrapper";

const initialCode = `// OmniLang Policy Document
// Define System Intent & Boundaries

INTENT: Melindungi keselamatan dan martabat pekerja
ACTOR:
- Primary: SafetySystem
- Secondary: Supervisor
CONTEXT:
- Domain: Factory Automation
- Lokasi: Assembly Line A
- Fase: Operation
RULE:
- IF WorkerDistance < 1m THEN StopMachineImmediate
CONSTRAINT:
- Legal: ISO-45001
- Ethical: HumanFirst
IMPACT:
- Benefit: Mencegah kecelakaan kerja
- Risk: Downtime produksi meningkat
TRACE:
- Moral: Value of Life
- Regulation: UU K3 No.1/1970
`;

const evaluatorDemoCode = `INTENT: Demonstrate evaluator features
ACTOR:
- Primary: Operator
CONTEXT:
- Domain: Testbed
RULE:
- IF Mode IN modes THEN ActivateProfile
- IF label IN ["a","b"] THEN RouteToClusterB
- IF sensor.temperature < 30 AND sensor.humidity < 90 THEN MaintainTrajectory
- FOR device IN sensors {
- IF device.status == "ok" THEN AddDevice
- IF device.flags[0] == "hot" THEN Escalate
}
IMPACT:
- Benefit: Demo coverage for evaluator features
TRACE:
- Moral: Value of Life
`;

const evaluatorDemoContext = `{
  "Mode": 2,
  "modes": [1, 2, 3],
  "label": "b",
  "sensor": {
    "temperature": 25,
    "humidity": 80
  },
  "sensors": [
    {
      "status": "ok",
      "flags": ["hot", "tagged"]
    },
    {
      "status": "offline",
      "flags": []
    }
  ]
}`;

const sampleLibrary = [
  { id: "safety", label: "Safety Sample", code: initialCode, context: "" },
  { id: "evaluator", label: "Evaluator Demo", code: evaluatorDemoCode, context: evaluatorDemoContext },
  {
    id: "demo",
    label: "Demo (Drone Logistics)",
    code: `INTENT: Mengelola operasional drone logistik secara otomatis
ACTOR:
- Primary: DronePilot
- Secondary: AirTrafficControl
CONTEXT:
- Domain: Logistics
- Lokasi: UrbanArea
- Fase: Flight
ASSUMPTION:
- GPS signal is stable
- Weather is clear
RULE:
- IF BatteryLevel < 20 THEN ReturnToHome
- IF ObstacleDetected == True THEN EmergencyBrake
CONSTRAINT:
- Legal: FAA Regulation Part 107
- Technical: Max altitude 400ft
IMPACT:
- Benefit: Efisiensi pengiriman
- Risk: Potensi tabrakan jika sensor gagal
TRACE:
- Regulation: https://www.faa.gov/uas
REVIEW:
- Interval: 30 Days
- Criteria: Accident rate < 0.1%`,
  },
  {
    id: "hello",
    label: "Hello",
    code: `INTENT: Menyapa dunia dan memperkenalkan OmniLang
ACTOR:
- Primary: User
- Secondary: OmniSystem
CONTEXT:
- Domain: General
- Lokasi: Global
- Fase: InitialContact
RULE:
- IF SystemStatus == "Ready" THEN Display "Hello, OmniLang!"
CONSTRAINT:
- Technical: Harus mendukung encoding UTF-8
IMPACT:
- Benefit: Memastikan sistem berfungsi
TRACE:
- Evidence: https://omnilang.dev/hello`,
  },
  {
    id: "loop_demo",
    label: "Loop Demo",
    code: `INTENT: Validasi Loop dalam Aturan Kebijakan

ACTOR:
- Primary: Admin
- Secondary: Sensor_Suhu

CONTEXT:
- Domain: Industri
- Lokasi: Gudang_A
- Fase: Operasional

RULE:
- IF Suhu > 50 THEN Aktifkan_Pendingin
- FOR agent IN Participants {
    - IF Status == 1 THEN Kirim_Notifikasi_Ke_agent
}
- WHILE Suhu > 45 {
    - IF Mode == 1 THEN Turunkan_Daya_Mesin
}

CONSTRAINT:
- Technical: Loop harus dibatasi untuk mencegah overhead.

IMPACT:
- Benefit: Otomasi aturan untuk banyak entitas.

TRACE:
- Evidence: https://omnilang.dev/spec/v1.0/loops`,
  },
  {
    id: "zero_trust_security",
    label: "Zero Trust Security",
    code: `INTENT: Mengamankan infrastruktur cloud dengan prinsip Zero Trust (Never Trust, Always Verify)

ACTOR:
- Primary: NetworkController
- Secondary: IdentityProvider
- Secondary: SecurityOperationCenter

CONTEXT:
- Domain: Cybersecurity
- Lokasi: Hybrid-Cloud-Environment
- Fase: Access-Request-Evaluation

ASSUMPTION:
- Semua perangkat pengguna memiliki sertifikat valid
- Sistem MFA (Multi-Factor Authentication) aktif

RULE:
- IF AuthenticationGaps > 0 THEN DenyAccess
- IF AnomalousBehaviorDetected == True THEN RevokeAllSessions
- FOR connection IN ActiveConnections {
    - IF TrafficEncryption == "None" THEN TerminateImmediate
    - WHILE AccessDuration > 8h {
        - IF ReAuthenticationFailed THEN LogSecurityIncident
    }
}

CONSTRAINT:
- Legal: NIST SP 800-207 Zero Trust Architecture
- Technical: Latensi verifikasi tidak boleh melebihi 200ms
- Moral: Privasi data pribadi admin harus tetap terjaga saat logging

IMPACT:
- Benefit: Mitigasi serangan lateral movement hingga 90%
- Risk: Ketidanyamanan pengguna akibat verifikasi berulang
- Trade-off: Keamanan ketat vs produktivitas karyawan

TRACE:
- Regulation: https://www.nist.gov/publications/zero-trust-architecture
- Evidence: Laporan uji penetrasi kuartal terakhir

REVIEW:
- Interval: 14 Days
- Criteria: Jumlah akun dengan akses istimewa yang tidak aktif = 0`,
  },
  {
    id: "factory_safety",
    label: "Factory Safety",
    code: `INTENT: Menjaga keselamatan pekerja pabrik secara real-time
ACTOR:
- Primary: SafetySystem
- Secondary: Supervisor
CONTEXT:
- Domain: Factory
- Lokasi: Line-3
- Fase: Operation
ASSUMPTION:
- Sensor kalibrasi terbaru
RULE:
- IF Distance < 1m THEN StopMachine
- IF Temperature > 52C THEN CoolDown
- FOR worker IN Workers {
    - IF WorkerDistance < 2m THEN SlowDown
}
- WHILE Temperature > 50C {
    - IF FanStatus == 0 THEN TurnOnFan
}
CONSTRAINT:
- Technical: Loop harus dibatasi dan unit harus dikenali
IMPACT:
- Benefit: Hindari kecelakaan dan overheating
- Risk: Downtime meningkat
TRACE:
- Evidence: https://omnilang.dev/spec/factory
REVIEW:
- Interval: Bulanan
- Criteria: Zero incident`,
  },
  {
    id: "evaluator_features",
    label: "Evaluator Features",
    code: evaluatorDemoCode,
    context: evaluatorDemoContext,
  },
  {
    id: "anti_money_laundering",
    label: "Anti Money Laundering",
    code: `INTENT: Mendeteksi dan mencegah pencucian uang dalam sistem perbankan digital

ACTOR:
- Primary: AMLMonitorSystem
- Secondary: ComplianceOfficer
- Secondary: CentralBank_API

CONTEXT:
- Domain: Finance-Stability
- Lokasi: Domestic-and-International
- Fase: Transaction-Settlement

ASSUMPTION:
- Daftar hitam (Sanction List) diperbarui setiap jam
- Kurs mata uang asing ditarik secara real-time

RULE:
- IF TransactionAmount > 10000USD THEN GenerateCTReport
- IF SenderCountry IN SanctionedList THEN BlockTransaction
- FOR account IN HighRiskAccounts {
    - IF VolumeIncrease > 300% THEN LockAccountTemporary
    - WHILE RiskPatternDetected == True {
        - IF InvestigationStatus == "Pending" THEN FreezeFunds
    }
}

CONSTRAINT:
- Legal: FATF Recommendations
- Technical: Pengecekan daftar sanksi harus < 50ms
- Ethical: Tidak boleh membekukan dana nasabah tanpa dasar bukti yang kuat

IMPACT:
- Benefit: Mencegah pendanaan aktivitas ilegal
- Risk: Komplain nasabah akibat pemblokiran yang salah (False Positive)
- Trade-off: Kecepatan transaksi vs ketelitian pengawasan

TRACE:
- Regulation: https://www.fatf-gafi.org/
- Evidence: Audit kepatuhan tahunan

REVIEW:
- Interval: Daily
- Criteria: Akurasi deteksi transaksi mencurigakan > 95%`,
  },
  {
    id: "green_data_center",
    label: "Green Data Center",
    code: `INTENT: Mengurangi jejak karbon pusat data melalui optimasi energi yang dinamis

ACTOR:
- Primary: EnergyManagementSystem
- Secondary: InfrastructureOps
- Secondary: CoolingController

CONTEXT:
- Domain: Sustainability-IT
- Lokasi: Data-Center-Alpha
- Fase: Operational-Load-Balancing

ASSUMPTION:
- PUE (Power Usage Effectiveness) diukur secara real-time
- Sumber energi terbarukan (solar/wind) tersedia pada siang hari

RULE:
- IF OutsideTemperature < 15C THEN UseFreeCooling
- IF RenewableEnergyAvailability > 80% THEN RunHeavyBatchJobs
- FOR rack IN ServerRacks {
    - IF CPU_Utilization < 10% THEN PutToSleepMode
    - WHILE AverageTemperature > 25C {
        - IF CoolingFanSpeed < 100% THEN IncreaseFanSpeed
    }
}

CONSTRAINT:
- Technical: SLA availability harus tetap 99.99%
- Environmental: Target emisi karbon < 2.0 kg CO2/MWh
- Legal: ISO 50001 Energy Management Standard

IMPACT:
- Benefit: Penurunan biaya operasional listrik hingga 25%
- Risk: Risiko degradasi hardware akibat siklus sleep/wake berlebih
- Trade-off: Penghematan energi vs performa responsif instan

TRACE:
- Regulation: https://www.iso.org/standard/69426.html
- Evidence: Sertifikasi bangunan hijau (LEED)

REVIEW:
- Interval: Monthly
- Criteria: Rata-rata PUE < 1.2`,
  },
  {
    id: "global_supply_chain",
    label: "Global Supply Chain",
    code: `INTENT: Meningkatkan transparansi dan etika dalam rantai pasok global

ACTOR:
- Primary: SupplyChainOrchestrator
- Secondary: LogisticsProvider
- Secondary: WarehouseManager

CONTEXT:
- Domain: Global-Trade
- Lokasi: International-Shipping-Lanes
- Fase: Sourcing-to-Delivery

ASSUMPTION:
- Tag RFID/IoT aktif pada setiap kontainer
- Data bea cukai tersedia secara elektronik

RULE:
- IF SupplierRating < 3 THEN SeekAlternativeSupplier
- IF ShippingPostponed > 48h THEN NotifyCustomerImmediate
- FOR container IN Shipments {
    - IF HumiditySensor > 70% THEN CheckProductIntegrity
    - WHILE ShipStatus == "AtPort" {
        - IF CustomsDelay > 5d THEN EscalateToPortAuthority
    }
}

CONSTRAINT:
- Legal: Incoterms 2020
- Ethical: Melarang kerja paksa dan eksploitasi anak di seluruh vendor
- Technical: Update status pengiriman minimal per 1 jam

IMPACT:
- Benefit: Kepuasan pelanggan meningkat melalui visibilitas real-time
- Risk: Gangguan geopolitik pada rute pelayaran tertentu
- Trade-off: Biaya audit vendor vs jaminan kepatuhan etika

TRACE:
- Regulation: https://iccwbo.org/resources-for-business/incoterms-rules/
- Evidence: Laporan audit keberlanjutan rantai pasok

REVIEW:
- Interval: Quarterly
- Criteria: Persentase vendor yang lulus audit etika = 100%`,
  },
  {
    id: "ai_ethics_governance",
    label: "AI Ethics Governance",
    code: `INTENT: Menjamin penggunaan AI yang etis, transparan, dan bebas bias dalam sistem rekrutmen

ACTOR:
- Primary: AutomatedRecruiterAI
- Secondary: EthicsCommittee
- Secondary: HRDepartment

CONTEXT:
- Domain: Corporate-HR
- Lokasi: Global-Offices
- Fase: Applicant-Screening

ASSUMPTION:
- Dataset pelatihan telah diaudit untuk representasi demografis
- API pihak ketiga untuk evaluasi skill tersedia

RULE:
- IF BiasScore > 0.05 THEN TriggerHumanReview
- IF TransparencyLevel < 100% THEN NotifyEthicsCommittee
- FOR applicant IN Applicants {
    - IF ProtectedAttributeDetected == True THEN RedactSensitiveData
    - WHILE ProcessingTime > 5s {
        - IF QueueLength > 100 THEN ScaleComputingResources
    }
}

CONSTRAINT:
- Legal: EU AI Act Compliance
- Ethical: Tidak boleh ada diskriminasi berdasarkan gender, usia, atau ras
- Technical: Model explainability must use SHAP or LIME

IMPACT:
- Benefit: Rekrutmen yang lebih adil dan objektif
- Risk: Potensi penolakan dari pelamar jika hasil AI tidak dijelaskan dengan baik
- Trade-off: Kecepatan pemrosesan vs akurasi deteksi bias

TRACE:
- Regulation: https://artificialintelligenceact.eu/
- Evidence: Dokumen audit model tahunan

REVIEW:
- Interval: 3 Months
- Criteria: Disparitas rekrutmen antar grup < 2%`,
  },
  {
    id: "actions_loop_showcase",
    label: "Actions & Loop Showcase",
    code: `INTENT: Orkestrasi respon insiden multi-region dengan aksi terarah
ACTOR:
- Primary: IncidentCommander
- Secondary: EdgeGateway_APAC
- Secondary: EdgeGateway_EMEA
- Secondary: SOC_Analyst
CONTEXT:
- Domain: SecurityOperations
- Lokasi: MultiRegion
- Fase: LiveOps
ASSUMPTION:
- Telemetry real-time tersedia
- Runbook disetujui tim legal
RULE:
- IF ThreatLevel >= 80 THEN Activate_Global_Runbook
- FOR agent IN IncidentActors {
    - IF agent.Status == "Degraded" THEN Dispatch_Remediation_to_agent
    - IF agent.BufferUsage > 75 THEN RateLimit_Traffic_for_agent
}
- WHILE AnomalyScore > 0.65 {
    - IF Region == "APAC" THEN Trigger_SwitchOver_APAC
    - IF Region == "EMEA" THEN Isolate_Segment_EMEA
}
CONSTRAINT:
- Technical: Loop dibatasi 10 iterasi untuk mencegah overload
- Legal: Ikuti SLA per region
IMPACT:
- Benefit: MTTR turun < 10 menit
- Risk: Switchover salah jika telemetry palsu
TRACE:
- Evidence: https://omnilang.dev/spec/v1.0/actions-loop-showcase
REVIEW:
- Interval: 14 Days
- Criteria: False positive < 3%`,
  },
  {
    id: "drone_patrol",
    label: "Drone Patrol",
    code: `INTENT: Memastikan patroli drone aman di area gudang
ACTOR:
- Primary: DroneFleet
- Secondary: OpsCenter
CONTEXT:
- Domain: Logistics
- Lokasi: Warehouse-7
- Fase: Patrol
ASSUMPTION:
- GPS tersedia
RULE:
- IF Battery < 20% THEN ReturnToBase
- IF WindSpeed > 25kmh THEN HoldPosition
- FOR drone IN Drones {
    - IF Altitude < 30m THEN IncreaseAltitude
    - WHILE Temperature > 45C {
        - IF Speed > 10kmh THEN ReduceSpeed
    }
}
- WHILE ObstacleDistance < 2m {
    - IF ObstacleSize > 0.5m THEN EmergencyBrake
}
CONSTRAINT:
- Technical: Nested loops must terminate
IMPACT:
- Benefit: Safe navigation
TRACE:
- Evidence: https://omnilang.dev/spec/drone
REVIEW:
- Interval: Mingguan
- Criteria: Zero collision`,
  },
  {
    id: "hospital_policy",
    label: "Hospital Policy",
    code: `INTENT: Menjaga kepatuhan etika dan keselamatan di rumah sakit
ACTOR:
- Primary: HospitalSystem
- Secondary: NurseSupervisor
CONTEXT:
- Domain: Healthcare
- Lokasi: ICU
- Fase: Operation
ASSUMPTION:
- Semua perangkat steril
RULE:
- IF PatientDistance < 0.5m THEN AlertStaff
- IF OxygenLevel < 92 THEN IncreaseO2
- FOR device IN Devices {
    - IF DeviceStatus == "Error" THEN NotifyTechnician
}
CONSTRAINT:
- Legal: HIPAA
- Ethical: PatientFirst
IMPACT:
- Benefit: Patient safety
- Risk: Alarm fatigue
TRACE:
- Regulation: https://omnilang.dev/spec/hipaa
REVIEW:
- Interval: Harian
- Criteria: Zero critical alarm missed`,
  },
  {
    id: "policy",
    label: "Minimal Policy",
    code: `INTENT: Melindungi keselamatan pekerja
ACTOR:
- Primary: SafetySystem
CONTEXT:
- Domain: Factory
RULE:
- IF Distance < 1m THEN StopMachine
CONSTRAINT:
- Ethical: HumanFirst
IMPACT:
- Benefit: SaveLife
TRACE:
- Moral: ValueLife`,
  },
  {
    id: "nested_loops_units",
    label: "Nested Loops Units",
    code: `INTENT: Stress test nested loops and mixed units
ACTOR:
- Primary: DroneFleet
- Secondary: OpsCenter
CONTEXT:
- Domain: Logistics
- Lokasi: Warehouse-7
- Fase: Patrol
RULE:
- IF Battery < 20% THEN ReturnToBase
- FOR drone IN Drones {
    - IF Altitude < 30m THEN IncreaseAltitude
    - WHILE Temperature > 45C {
        - IF Speed > 10kmh THEN ReduceSpeed
    }
}
- WHILE ObstacleDistance < 2m {
    - IF ObstacleSize > 0.5m THEN EmergencyBrake
}
CONSTRAINT:
- Technical: Nested loops must terminate
IMPACT:
- Benefit: Safe navigation
TRACE:
- Evidence: https://omnilang.dev/spec/nested`,
  },
  {
    id: "edge_units_nested",
    label: "Edge Units Nested",
    code: `INTENT: Menguji parser pada unit beragam dan loop dalam loop
ACTOR:
- Primary: TestHarness
- Secondary: QAEngineer
CONTEXT:
- Domain: Testing
- Lokasi: Lab
- Fase: Validation
ASSUMPTION:
- Data sintetik
RULE:
- IF Pressure > 2bar THEN Vent
- IF Speed > 30kmh THEN Brake
- FOR batch IN Batches {
    - IF Temp > 100C THEN Cool
    - WHILE Humidity > 60pct {
        - IF Distance < 50cm THEN Pause
    }
}
- WHILE Voltage > 240V {
    - IF Current > 5A THEN CutOff
}
CONSTRAINT:
- Technical: Parser harus tahan unit beragam
IMPACT:
- Benefit: Menangkap regresi lebih awal
TRACE:
- Evidence: https://omnilang.dev/spec/edge
REVIEW:
- Interval: Per rilisan
- Criteria: Zero parser panic`,
  },
  {
    id: "units_and_loops",
    label: "Units and Loops",
    code: `INTENT: Uji angka dengan unit dan loop bersarang
ACTOR:
- Primary: RobotArm
- Secondary: SafetySupervisor
CONTEXT:
- Domain: Manufacturing
- Lokasi: Line-1
- Fase: Operation
RULE:
- IF Distance < 1m THEN StopMachine
- IF Temperature > 52C THEN CoolDown
- FOR worker IN Workers {
    - IF WorkerDistance < 2m THEN SlowDown
}
- WHILE Temperature > 50C {
    - IF FanStatus == 0 THEN TurnOnFan
}
CONSTRAINT:
- Technical: Loop harus dibatasi dan unit harus dikenali
IMPACT:
- Benefit: Hindari kecelakaan dan overheating
TRACE:
- Evidence: https://omnilang.dev/spec/units`,
  },
  {
    id: "smart_city_traffic",
    label: "Smart City Traffic",
    code: `INTENT: Mengoptimalkan aliran trafik Smart City dan prioritas kendaraan darurat

ACTOR:
- Primary: TrafficControlSystem
- Secondary: EmergencyDispatch
- Secondary: PublicTransport

CONTEXT:
- Domain: SmartCity-Transportation
- Lokasi: Sektor-Pusat-Kota
- Fase: PeakHour-Adjustment

ASSUMPTION:
- Sensor IoT pada setiap persimpangan berfungsi
- Konektivitas 5G stabil untuk komunikasi antar-kendaraan

RULE:
- IF TrafficDensity > 80% THEN ActivateGreenWave
- IF EmergencyVehicleDetected == True THEN ClearPathImmediate
- FOR intersection IN Intersections {
    - IF QueueLength > 15m THEN ExtendGreenLight
    - WHILE EmergencyVehicleDistance < 500m {
        - IF LightStatus != "Red" THEN SetTrafficRedExceptEmergency
    }
}
- FOR v IN PublicBuses {
    - IF DelayInMinutes > 5 THEN PrioritizeBusLane
}

CONSTRAINT:
- Legal: UU Lalu Lintas No 22 Tahun 2009
- Ethical: Tidak boleh membahayakan pejalan kaki demi kendaraan darurat
- Technical: Max GreenLightDuration is 120s

IMPACT:
- Benefit: Pengurangan waktu respon darurat hingga 30%
- Risk: Potensi kemacetan di jalur non-prioritas
- Trade-off: Kelancaran transportasi umum vs efisiensi kendaraan pribadi

TRACE:
- Regulation: https://jdih.dephub.go.id/peraturan
- Evidence: Skenario simulasi trafik 2025

REVIEW:
- Interval: Setiap 24 Jam (Daily Audit)
- Criteria: Waktu respon ambulans rata-rata < 8 menit`,
  },
];

type ValidationResult = {
  errors: any[];
  rules: any[];
  actions: string[];
  stdout: string;
  stderr?: string;
  engine?: string;
  mode?: string;
  capabilities?: string[];
  compiler?: string;
};

export function OmniLangStudio() {
  const [code, setCode] = useState(initialCode);
  const [context, setContext] = useState("");
  const searchParams = useSearchParams();
  const { toast } = useToast();
  const [isValidating, setIsValidating] = useState(false);
  const [validationResult, setValidationResult] = useState<ValidationResult>({
    errors: [],
    rules: [],
    actions: [],
    stdout: "",
    stderr: "",
    engine: "",
    mode: "",
    capabilities: [],
    compiler: "",
  });

  const [sideWidth, setSideWidth] = useState(360);
  const [isResizingSide, setIsResizingSide] = useState(false);
  const startX = useRef(0);
  const startWidth = useRef(360);
  const MIN_SIDE = 260;
  const MAX_SIDE = 520;
  const [selectedSample, setSelectedSample] = useState("safety");
  const [fontSize, setFontSize] = useState(14);

  // VUI voice transcript handler
  const handleVoiceTranscript = (text: string) => {
    toast({
      title: "🎤 Suara terdeteksi",
      description: text,
    });
  };

  // VUI voice command handler
  const handleVoiceCommand = (command: string) => {
    if (command === "validate") {
      handleValidate();
      speak("Validasi sedang berjalan");
    } else if (command.startsWith("load")) {
      const name = command.replace("load ", "").trim();
      const found = sampleLibrary.find(
        (s) => s.id === name || s.label.toLowerCase().includes(name)
      );
      if (found) {
        setCode(found.code ?? "");
        setContext(found.context ?? "");
        speak(`Memuat contoh ${found.label}`);
      }
    } else if (command === "list") {
      speak(`Ada ${sampleLibrary.length} contoh policy tersedia`);
    }
  };

  // NUI handlers
  const handleFileDrop = (content: string, filename: string) => {
    setCode(content);
    toast({
      title: "📂 File dimuat",
      description: `${filename} (${content.split("\n").length} baris)`,
    });
  };

  const handlePinchIn = () => setFontSize((s) => Math.max(10, s - 1));
  const handlePinchOut = () => setFontSize((s) => Math.min(24, s + 1));
  const handleDoubleTap = () => handleValidate();

  useEffect(() => {
    const codeFromUrl = searchParams.get("code");
    if (!codeFromUrl) return;

    try {
      const decodedCode = atob(codeFromUrl);
      setCode(decodedCode);
    } catch (e) {
      console.error("Failed to decode code from URL:", e);
    }
  }, [searchParams]);

  useEffect(() => {
    if (!isResizingSide) return;

    const handleMove = (e: MouseEvent) => {
      const delta = e.clientX - startX.current;
      const next = Math.min(MAX_SIDE, Math.max(MIN_SIDE, startWidth.current + delta));
      setSideWidth(next);
    };

    const handleUp = () => setIsResizingSide(false);

    window.addEventListener("mousemove", handleMove);
    window.addEventListener("mouseup", handleUp);

    return () => {
      window.removeEventListener("mousemove", handleMove);
      window.removeEventListener("mouseup", handleUp);
    };
  }, [isResizingSide]);

  const handleValidate = async () => {
    setIsValidating(true);
    try {
      const res = await fetch("/api/validate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code, context }),
      });

      const data = await res.json();
      setValidationResult({
        errors: data?.errors ?? [],
        rules: data?.rules ?? [],
        actions: data?.actions ?? [],
        stdout: data?.stdout ?? "",
        stderr: data?.stderr ?? "",
        engine: data?.engine ?? "",
        mode: data?.mode ?? "",
        capabilities: data?.capabilities ?? [],
        compiler: data?.compiler ?? "",
      });

      const errorCount = data?.errors?.length ?? 0;
      const ruleCount = data?.rules?.length ?? 0;

      if (errorCount > 0) {
        toast({
          variant: "destructive",
          title: `Validation failed (${errorCount} issue${errorCount > 1 ? "s" : ""})`,
          description: data.errors
            .slice(0, 3)
            .map((e: any) => `L${e.line}: ${e.message}`)
            .join("; "),
        });
      } else {
        toast({
          variant: "default",
          title: "Validation passed",
          description: data?.actions?.length
            ? `${data.actions.length} action(s) triggered (${data?.engine ?? ""})`
            : ruleCount
              ? `${ruleCount} rule(s) parsed (${data?.engine ?? ""})`
              : "No rules detected",
        });
      }
    } catch (e) {
      console.error("Failed to validate", e);
      toast({
        variant: "destructive",
        title: "Validation error",
        description: "Failed to validate policy. Please try again.",
      });
    } finally {
      setIsValidating(false);
    }
  };

  return (
    <NUIWrapper
      onPinchIn={handlePinchIn}
      onPinchOut={handlePinchOut}
      onDoubleTap={handleDoubleTap}
      className="flex min-h-screen flex-col bg-slate-50 text-slate-900"
    >
      <Header code={code} onValidate={handleValidate} isValidating={isValidating} />

      {/* VUI Voice Button — Fixed top-right */}
      <div className="fixed top-4 right-4 z-40">
        <VUIButton onTranscript={handleVoiceTranscript} onCommand={handleVoiceCommand} />
      </div>

      <main className="flex flex-1 overflow-hidden">
        <aside className="hidden lg:flex w-80 shrink-0 flex-col border-r bg-white/80 backdrop-blur p-4 gap-4">
          <div className="space-y-2">
            <div className="flex items-center gap-2 text-[11px] font-semibold uppercase tracking-wide text-indigo-600">
              <span className="rounded-full bg-indigo-50 px-2 py-1">Validator Mode</span>
              <span className="rounded-full bg-amber-50 px-2 py-1 text-amber-700">Runtime Eval</span>
            </div>
            <h1 className="text-xl font-bold text-slate-900">OmniLang Studio</h1>
            <p className="text-sm text-slate-600 leading-relaxed">Parse, validate, dan evaluasi intent Anda dengan fitur IN, dot-path, dan loop iterator binding.</p>
            <p className="text-xs text-slate-500">Scope: intent validation. Compiler/stdlib/runtime produksi belum tersedia.</p>
          </div>
          <div className="flex flex-col gap-2">
            <label className="text-xs font-semibold text-slate-700">Load Example</label>
            <select
              value={selectedSample}
              onChange={(e) => setSelectedSample(e.target.value)}
              className="h-9 rounded border border-slate-300 bg-white px-2 text-sm text-slate-700"
            >
              {sampleLibrary.map((s) => (
                <option key={s.id} value={s.id}>{s.label}</option>
              ))}
            </select>
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  const found = sampleLibrary.find((s) => s.id === selectedSample);
                  if (found) {
                    setCode(found.code ?? "");
                    setContext(found.context ?? "");
                  }
                }}
              >
                Load Selected
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  const random = sampleLibrary[Math.floor(Math.random() * sampleLibrary.length)];
                  setSelectedSample(random.id);
                  setCode(random.code ?? "");
                  setContext(random.context ?? "");
                }}
              >
                Random
              </Button>
            </div>
            <Button size="sm" onClick={handleValidate} disabled={isValidating}>
              {isValidating ? "Validating..." : "Validate Now"}
            </Button>
          </div>
        </aside>

        <div className="flex-1 overflow-auto">
          <div className="mx-auto flex max-w-7xl flex-col gap-6 p-4 sm:p-6">
            <div className="flex flex-col gap-4 xl:flex-row xl:items-stretch">
              <FileDropZone onFileDrop={handleFileDrop} className="flex-1 min-w-0 rounded-xl border border-sky-500 bg-white shadow-sm p-4 space-y-3">
                <div className="flex items-center justify-between">
                  <div>
                    <h2 className="text-sm font-semibold text-slate-800">Policy Editor</h2>
                    <p className="text-xs text-slate-500">Tulis, tempel, atau drag & drop file .omni Anda.</p>
                  </div>
                  <span className="text-[10px] text-slate-400">{fontSize}px</span>
                </div>
                <div style={{ fontSize: `${fontSize}px` }}>
                  <CodeEditorPanel
                    code={code}
                    setCode={setCode}
                    validationResult={validationResult}
                    isValidating={isValidating}
                    onValidate={handleValidate}
                  />
                </div>
              </FileDropZone>

              <div
                className="hidden xl:flex w-2 cursor-col-resize items-stretch justify-center"
                onMouseDown={(e) => {
                  e.preventDefault();
                  startX.current = e.clientX;
                  startWidth.current = sideWidth;
                  setIsResizingSide(true);
                }}
                aria-label="Resize sidebar"
              >
                <div className="w-px bg-sky-400" />
              </div>

              <div className="flex flex-col gap-4 xl:w-[360px]" style={{ width: `${sideWidth}px` }}>
                <div className="rounded-xl border border-sky-500 bg-white shadow-sm p-4 space-y-3">
                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-sm font-semibold text-slate-700">Context JSON (opsional)</h3>
                      <p className="text-xs text-slate-500">Isi konteks runtime untuk evaluasi rules.</p>
                    </div>
                    <Button variant="ghost" size="sm" onClick={() => setContext("")}>Clear</Button>
                  </div>
                  <Textarea
                    value={context}
                    onChange={(e) => setContext(e.target.value)}
                    placeholder='{"Mode":2, ...}'
                    className="font-mono text-sm min-h-[160px] resize-vertical border border-sky-500/70"
                  />
                </div>

                <div className="rounded-xl border border-sky-500 bg-white shadow-sm p-4 flex-1 min-h-[240px]">
                  <div className="flex items-center justify-between pb-3 border-b border-sky-500/80">
                    <h3 className="text-sm font-semibold text-slate-700">Docs & Tips</h3>
                    <span className="text-[11px] font-medium text-indigo-600">v1.0</span>
                  </div>
                  <div className="h-full max-h-[480px] overflow-y-auto text-sm text-slate-600">
                    <Documentation />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* CUI Chatbot — Floating bubble */}
      <CUIPanel
        code={code}
        setCode={setCode}
        onValidate={handleValidate}
        sampleLibrary={sampleLibrary}
        setContext={setContext}
      />
    </NUIWrapper>
  );
}
