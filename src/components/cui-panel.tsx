"use client";

import { useState, useRef, useEffect } from "react";
import { Button } from "@/components/ui/button";

type ChatMessage = {
    role: "user" | "assistant" | "system";
    content: string;
    timestamp: Date;
};

interface CUIProps {
    code: string;
    setCode: (code: string) => void;
    onValidate: () => void;
    sampleLibrary: { id: string; label: string; code: string; context?: string }[];
    setContext: (ctx: string) => void;
}

export function CUIPanel({ code, setCode, onValidate, sampleLibrary, setContext }: CUIProps) {
    const [messages, setMessages] = useState<ChatMessage[]>([
        {
            role: "system",
            content: "🤖 Selamat datang di OmniLang CUI!\nKetik perintah seperti:\n• `help` — daftar perintah\n• `validate` — validasi policy aktif\n• `load <nama>` — muat contoh (e.g., `load hello`)\n• `list` — daftar contoh\n• `info` — info file aktif\n• `stats` — statistik kode",
            timestamp: new Date(),
        },
    ]);
    const [input, setInput] = useState("");
    const [isOpen, setIsOpen] = useState(false);
    const scrollRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (scrollRef.current) {
            scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
        }
    }, [messages]);

    const addMessage = (role: ChatMessage["role"], content: string) => {
        setMessages((prev) => [...prev, { role, content, timestamp: new Date() }]);
    };

    const processCommand = (cmd: string) => {
        const trimmed = cmd.trim().toLowerCase();
        addMessage("user", cmd);

        if (trimmed === "help") {
            addMessage("assistant",
                "📖 **Perintah OmniLang CUI:**\n" +
                "• `validate` — Validasi policy aktif\n" +
                "• `load <nama>` — Muat contoh policy\n" +
                "• `list` — Daftar semua contoh\n" +
                "• `info` — Info kode saat ini\n" +
                "• `stats` — Statistik kode\n" +
                "• `search <keyword>` — Cari di contoh\n" +
                "• `clear` — Bersihkan chat\n" +
                "• `voice` — Aktifkan input suara (VUI)"
            );
        } else if (trimmed === "validate") {
            addMessage("assistant", "🔄 Menjalankan validasi...");
            onValidate();
            addMessage("assistant", "✅ Validasi selesai. Cek panel output.");
        } else if (trimmed === "list") {
            const list = sampleLibrary.map((s, i) => `${i + 1}. ${s.label} (${s.id})`).join("\n");
            addMessage("assistant", `📂 Contoh policy tersedia:\n${list}`);
        } else if (trimmed.startsWith("load ")) {
            const name = trimmed.replace("load ", "").trim();
            const found = sampleLibrary.find(
                (s) => s.id === name || s.label.toLowerCase().includes(name)
            );
            if (found) {
                setCode(found.code ?? "");
                setContext(found.context ?? "");
                addMessage("assistant", `📂 Loaded: **${found.label}** (${found.code.split("\n").length} lines)`);
            } else {
                addMessage("assistant", `❓ Contoh "${name}" tidak ditemukan. Ketik \`list\` untuk daftar.`);
            }
        } else if (trimmed === "info") {
            const lines = code.split("\n").length;
            const chars = code.length;
            const isPolicy = code.trim().startsWith("INTENT:");
            addMessage("assistant",
                `📄 **Info Kode Aktif:**\n` +
                `• Lines: ${lines}\n` +
                `• Characters: ${chars}\n` +
                `• Type: ${isPolicy ? "Declarative Policy" : "Imperative Program"}\n` +
                `• Engine: ${isPolicy ? "Core Engine (Validator)" : "omc Compiler"}`
            );
        } else if (trimmed === "stats") {
            const lines = code.split("\n");
            const rules = lines.filter((l) => l.trim().startsWith("- IF ") || l.trim().startsWith("- FOR ") || l.trim().startsWith("- WHILE ")).length;
            const sections = ["INTENT", "ACTOR", "CONTEXT", "RULE", "CONSTRAINT", "IMPACT", "TRACE", "REVIEW"].filter(
                (s) => code.includes(`${s}:`)
            );
            addMessage("assistant",
                `📊 **Statistik:**\n` +
                `• Total baris: ${lines.length}\n` +
                `• Rules/Loops: ${rules}\n` +
                `• Sections: ${sections.join(", ") || "N/A"}\n` +
                `• Karakter: ${code.length}`
            );
        } else if (trimmed.startsWith("search ")) {
            const keyword = trimmed.replace("search ", "").trim();
            const results = sampleLibrary.filter(
                (s) => s.code.toLowerCase().includes(keyword) || s.label.toLowerCase().includes(keyword)
            );
            if (results.length > 0) {
                addMessage("assistant",
                    `🔍 Hasil pencarian "${keyword}":\n` +
                    results.map((r) => `• ${r.label} (${r.id})`).join("\n")
                );
            } else {
                addMessage("assistant", `🔍 Tidak ditemukan hasil untuk "${keyword}".`);
            }
        } else if (trimmed === "clear") {
            setMessages([{
                role: "system",
                content: "🤖 Chat dibersihkan. Ketik `help` untuk bantuan.",
                timestamp: new Date(),
            }]);
        } else {
            addMessage("assistant",
                `❓ Perintah tidak dikenal: "${cmd}"\nKetik \`help\` untuk daftar perintah.`
            );
        }
    };

    if (!isOpen) {
        return (
            <button
                onClick={() => setIsOpen(true)}
                className="fixed bottom-6 right-6 z-50 flex h-14 w-14 items-center justify-center rounded-full bg-gradient-to-br from-indigo-600 to-purple-600 text-white shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110"
                title="Open OmniLang Chat (CUI)"
            >
                <span className="text-2xl">💬</span>
            </button>
        );
    }

    return (
        <div className="fixed bottom-6 right-6 z-50 flex w-96 flex-col rounded-2xl border border-indigo-200 bg-white shadow-2xl overflow-hidden"
            style={{ height: "500px" }}
        >
            {/* Header */}
            <div className="flex items-center justify-between bg-gradient-to-r from-indigo-600 to-purple-600 px-4 py-3">
                <div className="flex items-center gap-2">
                    <span className="text-xl">🤖</span>
                    <div>
                        <h3 className="text-sm font-bold text-white">OmniLang CUI</h3>
                        <p className="text-[10px] text-indigo-200">Conversational Interface</p>
                    </div>
                </div>
                <button onClick={() => setIsOpen(false)} className="text-white/80 hover:text-white text-lg">✕</button>
            </div>

            {/* Messages */}
            <div ref={scrollRef} className="flex-1 overflow-y-auto p-3 space-y-3 bg-slate-50">
                {messages.map((msg, i) => (
                    <div key={i} className={`flex ${msg.role === "user" ? "justify-end" : "justify-start"}`}>
                        <div
                            className={`max-w-[85%] rounded-xl px-3 py-2 text-sm whitespace-pre-wrap ${msg.role === "user"
                                    ? "bg-indigo-600 text-white rounded-br-sm"
                                    : msg.role === "system"
                                        ? "bg-indigo-50 text-indigo-800 border border-indigo-200"
                                        : "bg-white text-slate-700 border border-slate-200 shadow-sm rounded-bl-sm"
                                }`}
                        >
                            {msg.content}
                        </div>
                    </div>
                ))}
            </div>

            {/* Input */}
            <div className="border-t border-slate-200 bg-white p-3">
                <form
                    onSubmit={(e) => {
                        e.preventDefault();
                        if (input.trim()) {
                            processCommand(input);
                            setInput("");
                        }
                    }}
                    className="flex gap-2"
                >
                    <input
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        placeholder="Ketik perintah... (help)"
                        className="flex-1 rounded-lg border border-slate-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                        autoFocus
                    />
                    <Button type="submit" size="sm" className="bg-indigo-600 hover:bg-indigo-700">
                        ↵
                    </Button>
                </form>
            </div>
        </div>
    );
}
