import { z } from "zod"

export const Fraction = z.object({
    numerator: z.number(),
    denumerator: z.union([z.number().positive(), z.number().negative()]),
})