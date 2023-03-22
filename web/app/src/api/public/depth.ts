import { z } from "zod";
import { Asset } from "~/types/asset";
import { PUBLIC_URL } from "./mod";
var EventSource = require('..')

const URL = PUBLIC_URL+"/depth"

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
    // (new EventSource(URL+"/sse")).onmessage = (event) => {
    //     console.log(event);
    //     console.log(Asset.parse(event.data()));
    // }
    
}

// console.log(sse());