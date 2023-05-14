import { Show } from "solid-js";
import { TermsModal } from "./Token/TermsModal";
import Crowdsale from "./Token/Crowdsale";
import {
  CrowdsaleProvider,
  useCrowdsale,
} from "~/utils/providers/CrowdsaleProvider";

export default function Token() {
  const crowdsale = useCrowdsale();

  return (
    <CrowdsaleProvider>
      <Crowdsale />
      <Show when={crowdsale.showModal}>
        <TermsModal />
      </Show>
    </CrowdsaleProvider>
  );
}
