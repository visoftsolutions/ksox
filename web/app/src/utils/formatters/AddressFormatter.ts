export class AddressFormatter {
  public static firstLastChars(str: string, beg: number, end: number): string {
    return `${str.slice(0, beg)}...${str.slice(-end)}`;
  }
}
