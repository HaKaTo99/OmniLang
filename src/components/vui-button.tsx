"use client";

import { useState, useRef, useEffect, useCallback } from "react";

interface VUIProps {
    onTranscript: (text: string) => void;
    onCommand?: (command: string) => void;
}

// Voice command patterns
const VOICE_COMMANDS: { pattern: RegExp; action: string }[] = [
    { pattern: /^(validasi|validate|jalankan|run)\b/i, action: "validate" },
    { pattern: /^(muat|load|buka|open)\s+(.+)/i, action: "load" },
    { pattern: /^(daftar|list|tampilkan)\b/i, action: "list" },
    { pattern: /^(info|informasi|status)\b/i, action: "info" },
    { pattern: /^(hapus|clear|bersihkan)\b/i, action: "clear" },
    { pattern: /^(bantuan|help|tolong)\b/i, action: "help" },
];

export function VUIButton({ onTranscript, onCommand }: VUIProps) {
    const [isListening, setIsListening] = useState(false);
    const [transcript, setTranscript] = useState("");
    const [isSupported, setIsSupported] = useState(true);
    const [volume, setVolume] = useState(0);
    const recognitionRef = useRef<any>(null);
    const animFrameRef = useRef<number>(0);

    useEffect(() => {
        // Check browser support
        const SpeechRecognition =
            (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;

        if (!SpeechRecognition) {
            setIsSupported(false);
            return;
        }

        const recognition = new SpeechRecognition();
        recognition.continuous = false;
        recognition.interimResults = true;
        recognition.lang = "id-ID"; // Indonesian + English
        recognition.maxAlternatives = 1;

        recognition.onresult = (event: any) => {
            const current = event.results[event.results.length - 1];
            const text = current[0].transcript;
            setTranscript(text);

            if (current.isFinal) {
                onTranscript(text);

                // Check for voice commands
                if (onCommand) {
                    for (const cmd of VOICE_COMMANDS) {
                        const match = text.match(cmd.pattern);
                        if (match) {
                            onCommand(cmd.action === "load" ? `load ${match[2] || ""}`.trim() : cmd.action);
                            break;
                        }
                    }
                }
            }
        };

        recognition.onerror = (event: any) => {
            console.error("Speech recognition error:", event.error);
            setIsListening(false);
        };

        recognition.onend = () => {
            setIsListening(false);
        };

        recognitionRef.current = recognition;

        return () => {
            if (recognitionRef.current) {
                try { recognitionRef.current.abort(); } catch { }
            }
            cancelAnimationFrame(animFrameRef.current);
        };
    }, [onTranscript, onCommand]);

    const toggleListening = useCallback(() => {
        if (!recognitionRef.current) return;

        if (isListening) {
            recognitionRef.current.stop();
            setIsListening(false);
            setTranscript("");
        } else {
            setTranscript("");
            recognitionRef.current.start();
            setIsListening(true);
        }
    }, [isListening]);

    if (!isSupported) {
        return (
            <button
                disabled
                className="flex items-center gap-1.5 rounded-lg bg-slate-100 px-3 py-2 text-xs text-slate-400 cursor-not-allowed"
                title="Browser tidak mendukung VUI (coba Chrome)"
            >
                🎤 VUI N/A
            </button>
        );
    }

    return (
        <div className="relative flex items-center gap-2">
            <button
                onClick={toggleListening}
                className={`flex items-center gap-1.5 rounded-lg px-3 py-2 text-xs font-medium transition-all duration-300 ${isListening
                        ? "bg-gradient-to-r from-red-500 to-pink-500 text-white shadow-lg shadow-red-200 animate-pulse"
                        : "bg-gradient-to-r from-violet-500 to-purple-500 text-white hover:shadow-lg hover:shadow-purple-200 hover:scale-105"
                    }`}
                title={isListening ? "Klik untuk berhenti mendengar" : "Klik untuk perintah suara (VUI)"}
            >
                <span className="text-base">{isListening ? "⏹" : "🎤"}</span>
                {isListening ? "Mendengar..." : "VUI"}
            </button>

            {isListening && transcript && (
                <div className="absolute top-full mt-2 left-0 z-50 w-72 rounded-xl border border-purple-200 bg-white p-3 shadow-xl">
                    <p className="text-xs text-slate-500 mb-1">🎤 Mendengar:</p>
                    <p className="text-sm font-medium text-slate-800 italic">&ldquo;{transcript}&rdquo;</p>
                </div>
            )}
        </div>
    );
}

// Text-to-Speech utility
export function speak(text: string, lang: string = "id-ID") {
    if (!("speechSynthesis" in window)) return;

    const utterance = new SpeechSynthesisUtterance(text);
    utterance.lang = lang;
    utterance.rate = 1.0;
    utterance.pitch = 1.0;
    utterance.volume = 0.8;

    window.speechSynthesis.cancel(); // Cancel any ongoing speech
    window.speechSynthesis.speak(utterance);
}
