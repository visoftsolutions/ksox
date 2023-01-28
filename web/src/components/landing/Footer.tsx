import Spacing from "../Spacing";
import ContactItems from "./footer/ContactItems";
import Text from "./footer/Text";

export default function Footer() {
  return (
    <section id="section-4">
      <footer class="flex flex-col items-center">
        <Text />
        <Spacing class="h-12" />
        <ContactItems />
      </footer>
    </section>
  );
}
