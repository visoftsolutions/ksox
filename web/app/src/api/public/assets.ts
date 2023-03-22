import { z } from "zod";
import { Pagination } from "~/api/mod";
import { Asset } from "~/types/asset";
import { PUBLIC_URL } from "./mod";

const URL = PUBLIC_URL+"/assets"

async function get() {
  return fetch(URL, {
    method: "GET",
    headers: {
        "Content-Type": "application/json",
    },
  })
  .then(r => r.json())
  .then(r => z.array(Asset).parse(r));
}

function sse() {
    console.log("event");
    // let stream = new ReadableStream<Asset>({
    //     start(controller) {
    //         new EventSource(URL+"/sse").onmessage = (event) => {
    //             console.log(event);
    //             console.log(Asset.parse(event.data()));
    //             controller.enqueue(Asset.parse(event.data()))
    //         }
            
    //     }
    // })
    
}