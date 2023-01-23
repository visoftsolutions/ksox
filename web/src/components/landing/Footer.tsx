import { A } from "solid-start";
import { DefaultProps } from "../interfaces";

export default function Footer(props: DefaultProps) {
  return (
    <section id="section-4">
      <footer class={`${props.class} flex flex-col items-center`}>
        <div class="flex flex-col items-center">
          <p class="text-footer-title font-bold text-white">Contact with us!</p>
          <p class="text-footer-title-light font-light text-text-faded">
            to learn more abut the project
          </p>
        </div>

        <div class="mt-[48px] flex justify-between gap-[72px]">
          <div class="flex flex-col items-center">
            <img class="mb-[12px]" src="/phone-icon.svg" />
            <span class="text-footer-element font-medium text-white">
              +48 601-356-047
            </span>
            <span class="text-footer-element-light font-light text-text-faded">
              call us maybe
            </span>
          </div>

          <div class="flex flex-col items-center">
            <img class="mb-[12px]" src="/mail-icon.svg" />
            <span class="text-footer-element font-medium text-white">
              ksox.exchange@proton.me
            </span>
            <span class="text-footer-element-light font-light text-text-faded">
              send us a message
            </span>
          </div>

          <div class="flex flex-col items-center">
            <img class="mb-[12px]" src="/mail-icon.svg" />
            <A
              class="text-footer-element font-medium text-white"
              href="https://twitter.com/KsoxExchange"
              target="_blank"
            >
              KsoxExchange
            </A>
            <span class="text-footer-element-light font-light text-text-faded">
              stay tuned
            </span>
          </div>
        </div>
      </footer>
    </section>
  );
}
