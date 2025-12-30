let inflight = 0;

export function beginRequest(maxInflight: number): boolean {
  if (inflight >= maxInflight) return false;
  inflight += 1;
  return true;
}

export function endRequest() {
  inflight = Math.max(0, inflight - 1);
}

export function getInflight(): number {
  return inflight;
}

export function resetInflight() {
  inflight = 0;
}
