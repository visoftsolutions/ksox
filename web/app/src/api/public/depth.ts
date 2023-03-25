import { z } from "zod";
import { SessionId } from "~/api/auth/mod";
import { DepthRequest, PriceLevel, PUBLIC_URL } from "./mod";

export const URL = PUBLIC_URL + "/depth";

export async function get(request: DepthRequest) {
  return fetch(URL, {
    method: "get",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  })
    .then((r) => r.json())
    .then((r) => z.array(PriceLevel).parse(r));
}

export function sse(): AsyncIterableIterator<PriceLevel> {
  let stream = new ReadableStream<PriceLevel>({
    start(controller) {
      new EventSource(URL + "/sse").onmessage = (event) => {
        console.log(event);
        console.log(PriceLevel.parse(event.data()));
        controller.enqueue(PriceLevel.parse(event.data()))
      }
    }
  });

  const reader = stream.getReader();
  return {
    async next() {
      const { done, value } = await reader.read();
      if (done) {
        return { value: value, done: true };
      } else {
        return { value: value };
      }
    },
    [Symbol.asyncIterator]: function () {
      return this;
    }
  };
}
