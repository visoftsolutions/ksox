import { Show } from "solid-js";
import { TermsModal } from "./Token/TermsModal";
import Crowdsale from "./Token/Crowdsale";
import {
  useCrowdsale,
} from "~/utils/providers/CrowdsaleProvider";

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
