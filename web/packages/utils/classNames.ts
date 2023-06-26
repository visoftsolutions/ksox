import clsx from "clsx";
import { twMerge } from "tailwind-merge";

export function classNames(...classNames: string[]) {
  return twMerge(clsx(...classNames));
}
