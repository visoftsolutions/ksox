import Spacing from "~/components/Spacing";
import ContactItems from "~/components/footer/ContactItems";
import Text from "~/components/footer/Text";

export default function Footer() {
  return (
    <section id="section-4">
      <footer class="flex flex-col justify-around md:flex-row">
        <Text />
        <Spacing class="h-12" />
        <ContactItems />
      </footer>
    </section>
  );
}
