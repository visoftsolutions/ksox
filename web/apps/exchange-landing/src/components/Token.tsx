import { Show } from "solid-js";
import { TermsModal } from "~/components/Token/TermsModal";
import Crowdsale from "~/components/Token/Crowdsale";
import { useCrowdsale } from "~/components/providers/CrowdsaleProvider";

export default function Token() {
  const crowdsale = useCrowdsale();

  return (
    <>
      <Crowdsale />
      <Show when={crowdsale.showModal}>
        <TermsModal />
      </Show>
    </>
  );
}
