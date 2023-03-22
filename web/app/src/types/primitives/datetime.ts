import { z } from "zod";

export const Datetime = z.string().datetime({ precision: 6 });