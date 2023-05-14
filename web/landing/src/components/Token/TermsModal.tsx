import { createSignal, onMount } from "solid-js";
import { joinPaths } from "solid-start/islands/server-router";
import { base } from "~/root";
import { setCrowdsale } from "~/utils/providers/CrowdsaleProvider";

export function TermsModal() {
  const [termsChecked, setTermsChecked] = createSignal<boolean>(false);

  let modalDOM!: HTMLDivElement;
  onMount(() => {
    modalDOM.focus();
  });

  return (
    <div
      class="fixed bottom-0 left-0 right-0 top-0 z-20 grid items-center justify-center bg-dark bg-opacity-80 font-lexend "
      onClick={(e) => {
        if (!modalDOM.contains(e.target)) {
          setCrowdsale({ showModal: false });
        }
      }}
    >
      <div
        class="m-auto grid h-[500px] w-[50%] grid-flow-row gap-8 rounded-xl bg-[#000033] p-4 text-text-1 outline-none"
        ref={modalDOM}
      >
        <div class="grid grid-cols-[auto_1fr_auto] items-center justify-center gap-4 text-xl font-bold">
          <img
            src={joinPaths(base, "/gfx/legal-agreement.svg")}
            width="30px"
            elementtiming={""}
            fetchpriority={"high"}
          />
          <div>Legal agreement</div>
          <img
            class="cursor-pointer"
            src={joinPaths(base, "/gfx/cancel.svg")}
            width="30px"
            elementtiming={""}
            fetchpriority={"high"}
            onClick={() => setCrowdsale({ showModal: false })}
          />
        </div>

        <div class="overflow-scroll rounded-md border-[1px] border-gray-700 p-2">
          Lorem Ipsum is simply dummy text of the printing and typesetting
          industry. Lorem Ipsum has been the industry's standard dummy text ever
          since the 1500s, when an unknown printer took a galley of type and
          scrambled it to make a type specimen book. It has survived not only
          five centuries, but also the leap into electronic typesetting,
          remaining essentially unchanged. It was popularised in the 1960s with
          the release of Letraset sheets containing Lorem Ipsum passages, and
          more recently with desktop publishing software like Aldus PageMaker
          including versions of Lorem Ipsum. Why do we use it? It is a long
          established fact that a reader will be distracted by the readable
          content of a page when looking at its layout. The point of using Lorem
          Ipsum is that it has a more-or-less normal distribution of letters, as
          opposed to using 'Content here, content here', making it look like
          readable English. Many desktop publishing packages and web page
          editors now use Lorem Ipsum as their default model text, and a search
          for 'lorem ipsum' will uncover many web sites still in their infancy.
          Various versions have evolved over the years, sometimes by accident,
          sometimes on purpose (injected humour and the like). Where does it
          come from? Contrary to popular belief, Lorem Ipsum is not simply
          random text. It has roots in a piece of classical Latin literature
          from 45 BC, making it over 2000 years old. Richard McClintock, a Latin
          professor at Hampden-Sydney College in Virginia, looked up one of the
          more obscure Latin words, consectetur, from a Lorem Ipsum passage, and
          going through the cites of the word in classical literature,
          discovered the undoubtable source. Lorem Ipsum comes from sections
          1.10.32 and 1.10.33 of "de Finibus Bonorum et Malorum" (The Extremes
          of Good and Evil) by Cicero, written in 45 BC. This book is a treatise
          on the theory of ethics, very popular during the Renaissance. The
          first line of Lorem Ipsum, "Lorem ipsum dolor sit amet..", comes from
          a line in section 1.10.32. The standard chunk of Lorem Ipsum used
          since the 1500s is reproduced below for those interested. Sections
          1.10.32 and 1.10.33 from "de Finibus Bonorum et Malorum" by Cicero are
          also reproduced in their exact original form, accompanied by English
          versions from the 1914 translation by H. Rackham.
        </div>

        <div class="mr-4 grid cursor-pointer grid-cols-[auto_1fr] items-center">
          <input
            id="terms-checkbox"
            type="checkbox"
            onChange={(e) => setTermsChecked(e.target.checked)}
            class="h-4 w-4 cursor-pointer accent-buttonbg_new"
          />
          <label
            for="terms-checkbox"
            class="ml-2 cursor-pointer select-none text-sm font-medium"
          >
            I agree to terms of KSXT Crowdsale
          </label>
        </div>

        <div
          class={`rounded-full p-[11px_32px] text-center font-lexend text-hero-button font-medium md:p-[16px_40px] 
          ${
            termsChecked()
              ? "token-linear-wipe-button cursor-pointer text-text-1 transition-opacity duration-100 hover:opacity-90"
              : "bg-gray-900 text-gray-700"
          }`}
          onClick={() => {
            if (termsChecked()) {
              setCrowdsale({ showModal: false });
            }
          }}
        >
          Continue
        </div>
      </div>
    </div>
  );
}
