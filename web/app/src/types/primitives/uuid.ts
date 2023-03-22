import { z } from "zod";

export const Uuid = z.string().uuid();