import { A } from "solid-start";
import Spacing from "~/components/Spacing";
import ContactItem from "./ContactItem";

export default function ContactItems() {
  return (
    <div class="flex flex-col justify-between lg:flex-row lg:gap-[4.5rem]">
      <ContactItem imageSrc="/gfx/phone-icon.svg">
        <p class="font-lexend text-footer-element font-medium text-text-1">
          +48 601-356-047
        </p>
        <p class="font-lexend text-footer-element-light font-light text-text-2">
          call us maybe
        </p>
      </ContactItem>

      <Spacing class="h-12" />

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
            stay tuned
          </p>
        </ContactItem>
      </A>
    </div>
  );
}
