export const rateBuckets = new Map<string, number[]>();

export function resetRateBuckets() {
  rateBuckets.clear();
}
