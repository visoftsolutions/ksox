import { base } from "~/root";
import Item from "./Footer/Item";
import ItemIcon from "./Footer/ItemIcon";
import { joinPaths } from "solid-start/islands/server-router";

export default function Footer() {
  return (
    <>
      <div class="pointer-events-none fixed bottom-4 left-10 right-10 grid items-center justify-end max-md:justify-center">
        <div class="pointer-events-auto grid grid-flow-col gap-4 rounded-full p-2 px-8 backdrop-blur-md">
          <a
            class="cursor-pointer"
            href="https://twitter.com/KsoxExchange"
            target="_blank"
          >
            <img
              src={joinPaths(base, "/gfx/twitter.svg")}
              alt="media"
              fetchpriority="auto"
              elementtiming="svg-image"
              width={40}
              height={40}
            />
          </a>
          <a
            class="cursor-pointer"
            href="https://discord.gg/d9qn83Qnbv"
            target="_blank"
          >
            <img
              src={joinPaths(base, "/gfx/discord.svg")}
              alt="media"
              fetchpriority="auto"
              elementtiming="svg-image"
              width={40}
              height={40}
            />
          </a>
          <a
            class="cursor-pointer"
            href="https://t.me/ksox_community"
            target="_blank"
          >
            <img
              src={joinPaths(base, "/gfx/telegram.svg")}
              alt="media"
              fetchpriority="auto"
              elementtiming="svg-image"
              width={40}
              height={40}
            />
          </a>
        </div>
      </div>
      <section id="contact">
        <footer>
          <div class="grid grid-cols-4 items-start justify-between gap-8 font-lexend text-text-1 max-lg:hidden">
            <div class="grid grid-flow-row gap-4">
              <div class="text-2xl font-bold">Product</div>
              <a href="/app" target="_blank">
                <Item
                  title="Exchange app"
                  description="Ksox Exchange trading platform"
                />
              </a>
              <a href="/whitepaper/ksox-whitepaper.pdf" target="_blank">
                <Item
                  title="Whitepaper"
                  description="Documentation about Cryptocurrency Exchange & Payment Processor"
                />
              </a>
              <a href="/token" target="_blank">
                <Item title="Token" description="Crowdsale section" />
              </a>
            </div>
            <div class="grid grid-flow-row gap-4">
              <div class="text-2xl font-bold">Legal</div>
              <Item
                title="Privacy policy"
                description="Our policies around privacy and data gathering"
              />
              <Item
                title="Cookies policy"
                description="Our policies around cookies"
              />
              <Item
                title="Crowdsale terms"
                description="Our policies around crowdsale and token"
              />
            </div>
            <div class="grid grid-flow-row gap-4">
              <div class="text-2xl font-bold">Technology</div>
              <a href="https://www.cairo-lang.org/" target="_blank">
                <Item
                  title="Cairo"
                  description="Turing-complete language for writing provable programs on blockchain"
                />
              </a>
              <a href="https://www.rust-lang.org/" target="_blank">
                <Item
                  title="Rust"
                  description="Multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency"
                />
              </a>
              <a href="https://www.solidjs.com/" target="_blank">
                <Item
                  title="SolidJs"
                  description="Simple and performant reactivity for building user interfaces."
                />
              </a>
              <a href="https://soliditylang.org/" target="_blank">
                <Item
                  title="Solidity"
                  description="Language designed for developing smart contracts"
                />
              </a>
            </div>
            <div class="grid grid-flow-row gap-4">
              <div class="text-2xl font-bold">Community</div>
              <a href="https://twitter.com/KsoxExchange" target="_blank">
                <ItemIcon
                  imageSrc="/gfx/twitter.svg"
                  title="KSOX | Twitter"
                  description="stay in touch"
                />
              </a>
              <a href="https://discord.gg/d9qn83Qnbv" target="_blank">
                <ItemIcon
                  imageSrc="/gfx/discord.svg"
                  title="KSOX | Discord"
                  description="join community"
                />
              </a>
              <a href="https://t.me/ksox_community" target="_blank">
                <ItemIcon
                  imageSrc="/gfx/telegram.svg"
                  title="KSOX | Telegram"
                  description="join community"
                />
              </a>
            </div>
          </div>
        </footer>
      </section>
    </>
  );
}
