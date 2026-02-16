"use client";

import { useEffect, useRef, useState, useCallback, type ReactNode } from "react";

interface NUIWrapperProps {
    children: ReactNode;
    onSwipeLeft?: () => void;
    onSwipeRight?: () => void;
    onSwipeUp?: () => void;
    onSwipeDown?: () => void;
    onPinchIn?: () => void;
    onPinchOut?: () => void;
    onDoubleTap?: () => void;
    className?: string;
}

type Point = { x: number; y: number };

export function NUIWrapper({
    children,
    onSwipeLeft,
    onSwipeRight,
    onSwipeUp,
    onSwipeDown,
    onPinchIn,
    onPinchOut,
    onDoubleTap,
    className = "",
}: NUIWrapperProps) {
    const containerRef = useRef<HTMLDivElement>(null);
    const touchStart = useRef<Point | null>(null);
    const lastTapTime = useRef(0);
    const initialPinchDistance = useRef<number | null>(null);

    const getDistance = (t1: Touch, t2: Touch): number => {
        return Math.hypot(t2.clientX - t1.clientX, t2.clientY - t1.clientY);
    };

    useEffect(() => {
        const el = containerRef.current;
        if (!el) return;

        const handleTouchStart = (e: TouchEvent) => {
            if (e.touches.length === 1) {
                touchStart.current = { x: e.touches[0].clientX, y: e.touches[0].clientY };

                // Double tap detection
                const now = Date.now();
                if (now - lastTapTime.current < 300) {
                    onDoubleTap?.();
                }
                lastTapTime.current = now;
            } else if (e.touches.length === 2) {
                initialPinchDistance.current = getDistance(e.touches[0], e.touches[1]);
            }
        };

        const handleTouchEnd = (e: TouchEvent) => {
            if (e.changedTouches.length === 1 && touchStart.current) {
                const dx = e.changedTouches[0].clientX - touchStart.current.x;
                const dy = e.changedTouches[0].clientY - touchStart.current.y;
                const absDx = Math.abs(dx);
                const absDy = Math.abs(dy);
                const threshold = 50;

                if (absDx > threshold || absDy > threshold) {
                    if (absDx > absDy) {
                        if (dx > 0) onSwipeRight?.();
                        else onSwipeLeft?.();
                    } else {
                        if (dy > 0) onSwipeDown?.();
                        else onSwipeUp?.();
                    }
                }
            }
            touchStart.current = null;
            initialPinchDistance.current = null;
        };

        const handleTouchMove = (e: TouchEvent) => {
            if (e.touches.length === 2 && initialPinchDistance.current !== null) {
                const currentDist = getDistance(e.touches[0], e.touches[1]);
                const delta = currentDist - initialPinchDistance.current;

                if (Math.abs(delta) > 30) {
                    if (delta > 0) onPinchOut?.();
                    else onPinchIn?.();
                    initialPinchDistance.current = currentDist;
                }
            }
        };

        el.addEventListener("touchstart", handleTouchStart, { passive: true });
        el.addEventListener("touchend", handleTouchEnd, { passive: true });
        el.addEventListener("touchmove", handleTouchMove, { passive: true });

        return () => {
            el.removeEventListener("touchstart", handleTouchStart);
            el.removeEventListener("touchend", handleTouchEnd);
            el.removeEventListener("touchmove", handleTouchMove);
        };
    }, [onSwipeLeft, onSwipeRight, onSwipeUp, onSwipeDown, onPinchIn, onPinchOut, onDoubleTap]);

    return (
        <div ref={containerRef} className={`touch-manipulation ${className}`}>
            {children}
        </div>
    );
}

// Drag & Drop wrapper for .omni files
interface FileDropZoneProps {
    children: ReactNode;
    onFileDrop: (content: string, filename: string) => void;
    className?: string;
}

export function FileDropZone({ children, onFileDrop, className = "" }: FileDropZoneProps) {
    const [isDragging, setIsDragging] = useState(false);

    const handleDragOver = useCallback((e: React.DragEvent) => {
        e.preventDefault();
        setIsDragging(true);
    }, []);

    const handleDragLeave = useCallback(() => {
        setIsDragging(false);
    }, []);

    const handleDrop = useCallback(
        async (e: React.DragEvent) => {
            e.preventDefault();
            setIsDragging(false);

            const files = Array.from(e.dataTransfer.files);
            const omniFile = files.find((f) => f.name.endsWith(".omni"));

            if (omniFile) {
                const text = await omniFile.text();
                onFileDrop(text, omniFile.name);
            }
        },
        [onFileDrop]
    );

    return (
        <div
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            className={`relative ${className} ${isDragging ? "ring-2 ring-indigo-400 ring-offset-2" : ""}`}
        >
            {children}
            {isDragging && (
                <div className="absolute inset-0 z-50 flex items-center justify-center rounded-xl bg-indigo-600/90 backdrop-blur-sm">
                    <div className="text-center text-white">
                        <span className="text-4xl">📂</span>
                        <p className="mt-2 text-lg font-bold">Drop .omni file here</p>
                        <p className="text-sm text-indigo-200">Drag & drop to load</p>
                    </div>
                </div>
            )}
        </div>
    );
}
