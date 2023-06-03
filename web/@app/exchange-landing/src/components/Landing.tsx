import Divider from "~/components/Divider";
import Spacing from "~/components/Spacing";
import Hero from "~/components/Landing/Hero";
import Section from "~/components/Landing/Section";

export default function Main() {
  return (
    <main class="flex-1">
      <Hero />

      <Spacing class="h-[4.5rem]" />
      <Divider />
      <Spacing class="h-12" />

      <Section sectionId="safety" sectionToId="zk-STARK" imagePath="/gfx/section01.webp" class="md:flex-row">
        <div class="font-lexend text-section-beginning font-bold text-secondary md:text-section-beginning-md ">KSOX is safe</div>

        <div class="font-lexend text-section-title font-semibold text-text-1 md:text-section-title-md">Self-custodial exchange, on-chain deposit treasury.</div>

        <div class="font-lexend text-section-main font-light text-text-2 md:text-section-main-md">
          The exchange gives users' full control over their funds. Because of the Treasury smart-contract user remains full-fledged owner of their wealth. Funds
          are allowed to be transferred only provided valid signature.
        </div>
      </Section>

      <Spacing class="h-16" />

      <Section sectionId="zk-STARK" sectionToId="technology-stack" imagePath="/gfx/section02.webp" class="md:flex-row-reverse">
        <div class="font-lexend text-section-beginning font-bold text-secondary md:text-section-beginning-md ">KSOX is zk-STARK powered</div>

        <div class="font-lexend text-section-title font-bold text-text-1 md:text-section-title-md">
          zk-STARK trail-blazing technology allowing fast, cheap and secure transfers.
        </div>

        <div class="font-lexend text-section-main font-light text-text-2 md:text-section-main-md">
          This allows to apply the security of fully decentralized exchange with responsiveness of centralized service. The Cairo-lang allows heavy computations
          handled off-chain to be boiled down into lightweight proof validated by smart-contract.
        </div>
      </Section>

      <Spacing class="h-16" />

      <Section sectionId="technology-stack" sectionToId="contact" imagePath="/gfx/section03.webp" class="md:flex-row">
        <div class="font-lexend text-section-beginning font-bold text-secondary md:text-section-beginning-md">KSOX is fast</div>

        <div class="font-lexend text-section-title font-bold text-text-1 md:text-section-title-md">
          Carefully chosen stack for speed and reliability. Rust and SolidJS together cover web parts.
        </div>

        <div class="font-lexend text-section-main font-light text-text-2 md:text-section-main-md">
          Rust ensures the exchange's backend is reliable and scalable. As compiled language it stands unsurpassed solution. User side powered by SolidJS, a
          framework that outruns its competitors. With Typescript both enforce desired quality.
        </div>
      </Section>

      <Spacing class="h-[4.5rem]" />
      <Divider />
      <Spacing class="h-12" />
    </main>
  );
}
