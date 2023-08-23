import Picture from "~/components/Atoms/Picture";
import SearchBar from "~/components/Atoms/SearchBar";
import {
  ColorMode,
  useColorMode,
} from "@packages/components/providers/ColorModeProvider";
import NewButton from "~/components/Atoms/Buttons/NewButton";

export default function Header(props: { class?: string }) {
  const colorMode = useColorMode();
  return (
    <div
      class={`grid grid-cols-[1fr_auto_1fr] grid-rows-[auto_auto] justify-center gap-4 ${props.class}`}
    >
      <div
        class="row-start-1 row-end-2 col-span-1 xl:hidden"
        onClick={() => {
          const enumValues = Object.values(ColorMode);
          colorMode.setColorMode(
            enumValues[
              (enumValues.indexOf(colorMode.colorMode()) + 1) %
                enumValues.length
            ],
          );
        }}
      >
        <Picture src="gfx/bitcoin_placeholder.png" alt="test" size={32} />
      </div>
      <div class="text-xl xl:text-3xl font-sans font-bold text-r-light-text dark:text-r-dark-text row-start-1 row-end-2 col-span-1 xl:col-span-2">
        Transfer
      </div>
      <NewButton class="justify-self-end row-start-1 row-end-2 col-span-1" />
      <div class="row-start-2 row-end-3 col-span-3">
        <SearchBar placeholder="Search" />
      </div>
    </div>
  );
}
