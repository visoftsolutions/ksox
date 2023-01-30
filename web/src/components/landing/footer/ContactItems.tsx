import { A } from "solid-start";
import Spacing from "~/components/Spacing";
import ContactItem from "./ContactItem";

export default function ContactItems() {
  return (
    <div class="flex flex-col justify-between lg:flex-row lg:gap-[4.5rem]">
      <ContactItem imageSrc="/gfx/mail-icon.svg">
        <p class="font-lexend text-footer-element font-medium text-text-1">
          ksox.exchange@proton.me
        </p>
        <p class="font-lexend text-footer-element-light font-light text-text-2">
          send us email
        </p>
      </ContactItem>

      <Spacing class="h-12" />

      <A href="https://twitter.com/KsoxExchange" target="_blank">
        <ContactItem imageSrc="/gfx/twitter-icon.svg">
          <p class="font-lexend text-footer-element font-medium text-text-1">
            KSOX - Exchange
          </p>
          <p class="font-lexend text-footer-element-light font-light text-text-2">
            keep in touch
          </p>
        </ContactItem>
      </A>
    </div>
  );
}
