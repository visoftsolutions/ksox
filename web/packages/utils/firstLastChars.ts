export default function firstLastChars(
  str: string,
  beg: number,
  end: number,
): string {
  return `${str.slice(0, beg)}...${str.slice(-end)}`;
}
