import Hero from "./Hero";
import Section from "./Section";

export default function Main() {
  return (
    <main class="flex-1">
      <Hero />

      <hr class="[border:1px_solid_#202020] mt-[144px] mb-[128px]" />

      <Section
        class="mb-[128px]"
        sectionId="section-1"
        sectionToId="section-2"
        imagePath="/about01.png"
        imageClass="w-[590px] h-[480px] px-[20px]"
        text0="Why KSOX is so good?"
        text1="Self-custodial exchange, on-chain deposit treasury."
      >
        The exchange gives users' full control over their funds. Because of the
        Treasury smart-contract user remains full-fledged owner of their wealth.
        Funds are allowed to be transferred only provided valid signature.
      </Section>

      <Section
        class="mb-[128px] flex-row-reverse"
        sectionId="section-2"
        sectionToId="section-3"
        imagePath="/about02a.png"
        imageClass="w-[590px] h-[480px] px-[20px]"
        text0="KSOX is zk-STARK powered"
        text1="zk-STARK trail-blazing technology allowing fast, cheap and secure transfers."
      >
        This allows to apply the security of fully decentralized exchange with
        responsiveness of centralized service. The Cairo-lang allows heavy
        computations handled off-chain to be boiled down into lightweight proof
        validated by smart-contract.
      </Section>

      <Section
        class="mb-[128px]"
        sectionId="section-3"
        imagePath="/about03.png"
        imageClass="w-[590px] h-[377px]"
        text0="Why KSOX is so good?"
        text1="Carefully chosen stack for speed and reliability. Rust and SolidJS together cover web parts."
      >
        Rust ensures the exchange's backend is reliable and scalable. As
        compiled language it stands unsurpassed solution. User side powered by
        SolidJS, a framework that outruns its competitors. With Typescript both
        enforce desired quality.
      </Section>

      <hr class="[border:1px_solid_#202020] mt-[144px] mb-[128px]" />
    </main>
  );
}
