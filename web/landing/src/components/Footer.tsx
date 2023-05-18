import Spacing from "~/components/Spacing";
import ContactItems from "~/components/Footer/ContactItems";
import Text from "~/components/Footer/Text";

export default function Footer() {
  return (
    <section id="contact">
      <footer>
        <div class="flex flex-col items-center justify-around">
          <Text />
          <Spacing class="h-12" />
          <ContactItems />
        </div>
      </footer>
    </section>
  );
}
