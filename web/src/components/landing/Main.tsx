import Divider from "../Divider";
import Spacing from "../Spacing";
import Hero from "./main/Hero";
import Section from "./main/Section";

export default function Main() {
  return (
    <main class="flex-1">
      <Hero />

      <Spacing class="h-[4.5rem]" />
      <Divider />
      <Spacing class="h-12" />

      <Section
        sectionId="section-1"
        sectionToId="section-2"
        imagePath="/gfx/about01.png"
        class="md:flex-row"
      >
        <div class="font-lexend text-section-beginning font-bold text-secondary">
          KSOX is safe
        </div>

        <div class="font-lexend text-section-title font-bold text-text-1">
          Self-custodial eXchange, on-chain deposit treasury.
        </div>

        <div class="font-lexend text-section-main font-light text-text-2">
          The exchange gives users' full control over their funds. Because of
          the Treasury smart-contract user remains full-fledged owner of their
          wealth. Funds are allowed to be transferred only provided valid
          signature.
        </div>
      </Section>

      <Spacing class="h-16" />

      <Section
        sectionId="section-2"
        sectionToId="section-3"
        imagePath="/gfx/about02.png"
        class="md:flex-row-reverse"
      >
        <div class="font-lexend text-section-beginning font-bold text-secondary">
          KSOX is zk-STARK powered
        </div>

        <div class="font-lexend text-section-title font-bold text-text-1">
          zk-STARK trail-blazing technology allowing fast, cheap and secure
          transfers.
        </div>

        <div class="font-lexend text-section-main font-light text-text-2">
          This allows to apply the security of fully decentralized exchange with
          responsiveness of centralized service. The Cairo-lang allows heavy
          computations handled off-chain to be boiled down into lightweight
          proof validated by smart-contract.
        </div>
      </Section>

      <Spacing class="h-16" />

      <Section
        sectionId="section-3"
        sectionToId="section-4"
        imagePath="/gfx/about03.png"
        class="md:flex-row"
      >
        <div class="font-lexend text-section-beginning font-bold text-secondary">
          KSOX is fast
        </div>

        <div class="font-lexend text-section-title font-bold text-text-1">
          Carefully chosen stack for speed and reliability. Rust and SolidJS
          together cover web parts.
        </div>

        <div class="font-lexend text-section-main font-light text-text-2">
          Rust ensures the exchange's backend is reliable and scalable. As
          compiled language it stands unsurpassed solution. User side powered by
          SolidJS, a framework that outruns its competitors. With Typescript
          both enforce desired quality.
        </div>
      </Section>

      <Spacing class="h-[4.5rem]" />
      <Divider />
      <Spacing class="h-12" />
    </main>
  );
}
