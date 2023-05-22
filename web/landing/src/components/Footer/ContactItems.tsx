import ContactItem from "~/components/Footer/ContactItem";

export default function ContactItems() {
  return (
    <div class="grid grid-cols-1 grid-rows-3 items-center gap-16 max-md:grid-cols-1 max-md:grid-rows-4">
      <a
        href="https://twitter.com/KsoxExchange"
        target="_blank"
        class="col-start-1 col-end-2 row-start-1 row-end-2 max-md:col-start-1 max-md:col-end-2 max-md:row-start-2 max-md:row-end-3"
      >
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

      <a
        href="https://discord.gg/d9qn83Qnbv"
        target="_blank"
        class="col-start-1 col-end-2 row-start-2 row-end-3 max-md:col-start-1 max-md:col-end-2 max-md:row-start-3 max-md:row-end-4"
      >
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

      <a
        href="https://t.me/ksox_community"
        target="_blank"
        class="col-start-1 col-end-2 row-start-3 row-end-4 max-md:col-start-1 max-md:col-end-2 max-md:row-start-4 max-md:row-end-5"
      >
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
