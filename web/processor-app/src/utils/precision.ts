export function formatTemplate(precission: number) {
  if (precission > 0) {
    return "0,0." + "0".repeat(precission);
  } else {
    return "0,0";
  }
}
