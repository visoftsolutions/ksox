import Spacing from "~/components/Spacing";
import ContactItem from "~/components/footer/ContactItem";

export default function ContactItems() {
  return (
    <div>
      <div>
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

      <a href="https://twitter.com/KsoxExchange" target="_blank">
        <ContactItem imageSrc="/gfx/twitter-icon.svg">
          <div>
            <p class="font-lexend text-footer-element font-medium text-text-1">
              KSOX | Twitter
            </p>
            <p class="text-center font-lexend text-footer-element-light font-light text-text-2 md:text-start">
              stay in touch
            </p>
          </div>
        </ContactItem>
      </a>

      <Spacing class="h-12" />

      <a href="https://discord.gg/u5ZGJ9mY" target="_blank">
        <ContactItem imageSrc="/gfx/discord-icon.svg">
          <div>
            <p class="font-lexend text-footer-element font-medium text-text-1">
              KSOX | Discord
            </p>
            <p class="text-center font-lexend text-footer-element-light font-light text-text-2 md:text-start">
              join community
            </p>
          </div>
        </ContactItem>
      </a>

      <Spacing class="h-12" />

      <a href="https://t.me/ksox_community" target="_blank">
        <ContactItem imageSrc="/gfx/telegram-icon.svg">
          <div>
            <p class="font-lexend text-footer-element font-medium text-text-1">
              KSOX | Telegram
            </p>
            <p class="text-center font-lexend text-footer-element-light font-light text-text-2 md:text-start">
              join community
            </p>
          </div>
        </ContactItem>
      </a>
    </div>
  );
}
