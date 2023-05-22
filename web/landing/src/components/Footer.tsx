import Spacing from "~/components/Spacing";
import ContactItems from "~/components/Footer/ContactItems";
import Text from "~/components/Footer/Text";
import Divider from "./Divider";
import { A } from "@solidjs/router";

export default function Footer() {
  return (
    <section id="contact">
      <footer>
        <div class="grid grid-cols-[auto_1fr] grid-rows-[auto_auto] gap-x-48 gap-y-8">
          <div class="col-start-1 col-end-2 row-start-1 row-end-2">
            <Text />
          </div>

          <div class="col-start-1 col-end-2 row-start-2 row-end-3">
            <ContactItems />
          </div>

          <div class="col-start-2 col-end-3 row-start-1 row-end-2 grid grid-cols-3 items-center justify-between self-center font-lexend text-2xl font-bold text-text-1">
            <div class="col-start-1 col-end-2 row-start-1 row-end-2 text-left">
              Product
            </div>
            <div class="col-start-2 col-end-3 row-start-1 row-end-2 text-left">
              Legal
            </div>
            <div class="col-start-3 col-end-4 row-start-1 row-end-2 text-left">
              Technology
            </div>
          </div>

          <div class="col-start-2 col-end-3 row-start-2 row-end-3 grid grid-cols-3 justify-center font-lexend font-normal text-gray-300">
            <div class="col-start-1 col-end-2 row-start-1 row-end-2">
              <div class="grid grid-flow-row-dense gap-2">
                <a
                  target="_blank"
                  href="/app"
                  class="cursor-pointer hover:text-gray-50"
                >
                  Exchange app
                </a>
                <a
                  target="_blank"
                  href="/whitepaper/ksox-whitepaper.pdf"
                  class="cursor-pointer hover:text-gray-50"
                >
                  Whitepaper
                </a>
                <div class="cursor-pointer hover:text-gray-50">Roadmap</div>
                <A href="/token" class="cursor-pointer hover:text-gray-50">
                  Token
                </A>
              </div>
            </div>
            <div class="col-start-2 col-end-3 row-start-1 row-end-2">
              <div class="grid grid-flow-row-dense gap-2">
                <div class="cursor-pointer hover:text-gray-50">
                  Privacy policy
                </div>
                <div class="cursor-pointer hover:text-gray-50">
                  Cookies policy
                </div>
                <div class="cursor-pointer hover:text-gray-50">
                  Crowdsale terms
                </div>
              </div>
            </div>
            <div class="col-start-3 col-end-4 row-start-1 row-end-2">
              <div class="grid grid-flow-row-dense gap-2">
                <a
                  target="_blank"
                  href="https://www.cairo-lang.org/"
                  class="cursor-pointer hover:text-gray-50"
                >
                  Cairo
                </a>
                <a
                  target="_blank"
                  href="https://www.rust-lang.org/"
                  class="cursor-pointer hover:text-gray-50"
                >
                  Rust
                </a>
                <a
                  target="_blank"
                  href="https://www.solidjs.com/"
                  class="cursor-pointer hover:text-gray-50"
                >
                  SolidJs
                </a>
              </div>
            </div>
          </div>
        </div>
        <div>
          <div>Maintained by the KSOX Team</div>
          <div></div>
        </div>
      </footer>
    </section>
  );
}
