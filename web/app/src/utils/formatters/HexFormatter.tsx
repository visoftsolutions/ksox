import { JSXElement } from "solid-js";

export interface Colors {
  prefixColor?: string;
  lowerColor?: string;
  upperColor?: string;
  numberColor?: string;
}

export class HexFormatter {
  public static hexPrefixRemove(str: string): string {
    return str.slice(2);
  }

  public static hexPrefixAdd(str: string): string {
    return "0x" + str;
  }

  public static hexColored(str: string, colors: Colors): JSXElement {
    const lower = /[a-f]/;
    const upper = /[A-F]/;
    const number = /\d/;

    return (
      <>
        <span style={{ color: colors.prefixColor }}>{str.substring(0, 2)}</span>
        {Array.from(str)
          .slice(2)
          .map((char) => {
            let ret: JSXElement;
            if (colors.lowerColor && char.match(lower)?.length) {
              ret = <span style={{ color: colors.lowerColor }}>{char}</span>;
            } else if (colors.upperColor && char.match(upper)?.length) {
              ret = <span style={{ color: colors.upperColor }}>{char}</span>;
            } else if (colors.numberColor && char.match(number)?.length) {
              ret = <span style={{ color: colors.numberColor }}>{char}</span>;
            } else {
              ret = char;
            }
            return ret;
          })}
      </>
    );
  }
}
