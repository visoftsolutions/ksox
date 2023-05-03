import Spacing from "~/components/Spacing";
import ContactItems from "~/components/footer/ContactItems";
import Text from "~/components/footer/Text";

export default function Footer() {
  return (
    <section id="section-4">
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
