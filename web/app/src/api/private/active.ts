import { z } from "zod";
import { Pagination } from "~/api/models";
import { SessionId } from "~/api/auth/models"
import { Order } from "~/types/order";

async function get(session: SessionId, pagination: Pagination) {
    let a = Order.safeParse({})
    return [a];
}