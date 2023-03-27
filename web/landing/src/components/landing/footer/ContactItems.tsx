import { A } from "solid-start";
import Spacing from "~/components/Spacing";
import ContactItem from "./ContactItem";

export default function ContactItems() {
  return (
    <div>
      <div class="flex flex-col items-center justify-center md:flex-col md:justify-start md:gap-[1.12rem] ">
        <ContactItem imageSrc="/gfx/mail-icon.svg">
          <div>
            <p class="font-lexend text-footer-element font-medium text-text-1">
              ksox.exchange@proton.me
            </p>
            <p class="text-center font-lexend text-footer-element-light font-light text-text-2 md:text-start">
              send us email
            </p>
          </div>
        </ContactItem>
      </div>

      <Spacing class="h-12" />

      <A href="https://twitter.com/KsoxExchange" target="_blank">
        <ContactItem imageSrc="/gfx/twitter-icon.svg">
          <div>
            <p class="font-lexend text-footer-element font-medium text-text-1">
              KSOX - Exchange
            </p>
            <p class="text-center font-lexend text-footer-element-light font-light text-text-2 md:text-start">
              keep in touch
            </p>
          </div>
        </ContactItem>
      </A>

      <Spacing class="h-12" />

      <A href="https://t.me/ksox_community" target="_blank">
        <ContactItem imageSrc="/gfx/telegram-icon.svg">
          <div>
            <p class="font-lexend text-footer-element font-medium text-text-1">
              KSOX | Community
            </p>
            <p class="text-center font-lexend text-footer-element-light font-light text-text-2 md:text-start">
              join community
            </p>
          </div>
        </ContactItem>
      </A>
    </div>
  );
}
