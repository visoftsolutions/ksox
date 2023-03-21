export class NumberFormatter {
  public static truncateNumber(number: number, digits: number): string {
    let ret: string;
    const range = Number(number.toExponential(digits).slice(-2));
    if (range < 1) {
      if (number < Math.pow(10, -(digits - 4))) {
        ret = number.toExponential(digits - 4);
      } else {
        ret = number.toFixed(digits - 1);
      }
    } else {
      if (range == digits - 1) {
        ret = number.toPrecision(digits);
      } else if (range < digits - 1) {
        ret = number.toPrecision(digits);
      } else {
        ret = number.toExponential(digits - 1);
      }
    }
    return ret;
  }

  public static humanReadable(number: number): string {
    let str = number.toString();
    const match = str.match(/\./);
    const index = match?.index ? match.index : str.length;
    for (let i = index - 3; i > 0; i -= 3) {
      str = str.substring(0, i) + "," + str.substring(i);
    }
    return str;
  }
}
