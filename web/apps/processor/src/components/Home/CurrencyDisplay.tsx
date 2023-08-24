import { useSelectedAsset } from "~/components/providers/SelectedAssetProvider";
import { joinPaths } from "solid-start/islands/server-router";
import Picture from "~/components/Atoms/Picture";
import { api, base } from "~/root";
import ArrowDownButton from "~/components/Atoms/Buttons/ArrowDownButton";
import {
  Index,
  Setter,
  Show,
  createMemo,
  createSignal,
  onCleanup,
  onMount,
} from "solid-js";
import HalfScreenModal from "~/components/Modals/HalfScreenModal";
import Currency from "~/components/Atoms/Currency";
import { useAssets } from "@packages/components/providers/AssetsProvider";
import subscribeEvents from "@packages/utils/subscribeEvents";
import { useSession } from "@packages/components/providers/SessionProvider";
import { usePrecision } from "@packages/components/providers/PrecisionProvider";
import params from "@packages/utils/params";
import { Valut } from "@packages/types/valut";
import { format } from "numerable";
import { Fraction, ev } from "@packages/types/primitives/fraction";
import { formatTemplate } from "@packages/utils/precision";
import { Asset } from "@packages/types/asset";
import { Dynamic } from "solid-js/web";
import { SessionResponse } from "@packages/components/providers/SessionProvider/models";

export default function CurrencyDisplay() {
  const session = useSession();
  const precision = usePrecision();
  const assets = useAssets();
  const { selectedAsset, setSelectedAsset } = useSelectedAsset();

  const [modal, setModal] = createSignal<boolean>(false);
  const assetsList = createMemo(() => [...assets().values()]);

  return (
    <div>
      <Dynamic
        component={CreateCurrencyDisplayAssetView(
          precision(),
          session(),
          selectedAsset(),
          setModal,
        )}
      />
      <Show when={modal()}>
        <HalfScreenModal close={() => setModal(false)}>
          <Index each={assetsList()} fallback={<div>No items</div>}>
            {(item) => (
              <Currency
                asset={item()}
                selected={selectedAsset() == item()}
                onClick={() => {
                  setSelectedAsset(item());
                }}
              />
            )}
          </Index>
        </HalfScreenModal>
      </Show>
    </div>
  );
}

const CreateCurrencyDisplayAssetView = (
  precision: number,
  session: SessionResponse | undefined,
  selectedAsset: Asset | undefined,
  setModal: Setter<boolean>,
) => {
  return () => {
    const [balance, setBalance] = createSignal<Fraction | null>(null);
    let eventsource: EventSource | undefined;

    onMount(async () => {
      if (session && selectedAsset && precision) {
        eventsource = await subscribeEvents(
          `${api}/private/balance`,
          params({ asset_id: selectedAsset.id }),
          params({ asset_id: selectedAsset.id }),
          (data) => {
            setBalance(Fraction.parse(Valut.parse(data).balance.Finite));
          },
        );
      }
    });

    onCleanup(() => {
      eventsource?.close();
    });

    return (
      <div class="px-4 pt-4">
        <div class="flex flex-row justify-between">
          <div>
            <div class="flex flex-row items-center">
              <p class="text-bold font-sans text-3xl text-r-light-text dark:text-r-dark-text">
                {balance()
                  ? format(ev(balance()!), formatTemplate(precision ?? 3))
                  : "---"}
              </p>
              <p class="pl-2 font-sans text-3xl text-r-dark-secondary-text">
                {selectedAsset?.symbol}
              </p>
              <ArrowDownButton class="m-1" onClick={() => setModal(true)} />
            </div>
            <p class="text-sans text-sm text-r-dark-secondary-text">
              {selectedAsset?.name}
            </p>
          </div>
          <Picture
            src={joinPaths(
              base,
              "/gfx/asset_icons/" +
                selectedAsset?.symbol.toLowerCase() +
                ".svg",
            )}
            alt="test"
            skeleton={selectedAsset == undefined}
          />
        </div>
      </div>
    );
  };
};
