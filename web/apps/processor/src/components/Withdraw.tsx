import {
  Component,
  Show,
  createEffect,
  createMemo,
  createSignal,
  onCleanup,
} from 'solid-js'
import { useAssets } from './providers/AssetsProvider'
import { Asset } from '@packages/types/asset'
import { joinPaths } from 'solid-start/islands/server-router'
import { api, base } from '~/root'
import { AVAILABLE_CHAINS } from '@packages/components/providers/WalletProvider/chains'
import { unwrap } from 'solid-js/store'
import { useWallet } from '@packages/components/providers/WalletProvider'
import { CreateProccessorDeposit } from './Asset/DepositCore'
import { usePrecision } from '@packages/components/providers/PrecisionProvider'
import { Dynamic } from 'solid-js/web'
import { Fraction, ev } from '@packages/types/primitives/fraction'
import { useSession } from '@packages/components/providers/SessionProvider'
import subscribeEvents from '@packages/utils/subscribeEvents'
import params from '@packages/utils/params'
import { Valut } from '@packages/types/valut'
import { format } from 'numerable'
import { formatTemplate } from '@packages/utils/precision'

export const Withdraw: Component<{}> = ({}) => {
  const assets = useAssets()
  const wallet = useWallet()
  const session = useSession()
  const precision = usePrecision()
  const assetsList = createMemo(() => [...assets().values()])

  const [selectedAsset, setSelectedAsset] = createSignal<Asset | undefined>()

  const assetMap = createMemo(() => {
    const map = new Map()
    assetsList().forEach((asset) => map.set(asset.id, asset))
    return map
  })

  const handleChangeCoin = (event: Event) => {
    const assetId = (event.target as HTMLSelectElement).value
    setSelectedAsset(assetMap().get(assetId))
  }

  createEffect(() => {
    if (assetsList().length > 0) {
      setSelectedAsset(assetsList()[0])
    }
  })

  const [balance, setBalance] = createSignal<Fraction | undefined>(undefined)

  let eventsource: EventSource | undefined

  createEffect(async () => {
    if (session() && selectedAsset() && precision()) {
      const asset = selectedAsset()

      eventsource = await subscribeEvents(
        `${api}/private/balance`,
        params({ asset_id: asset!.id }),
        params({ asset_id: asset!.id }),
        (data) => {
          setBalance(Valut.parse(data).balance.Finite)
        }
      )
    }
  })

  onCleanup(() => {
    eventsource?.close()
  })

  return (
    <div class="p-4">
      <h1 class="text-2xl font-semibold mb-6 text-center">Withdraw funds</h1>
      <div class="flex gap-4 items-center mb-4">
        <div class="relative inline-block w-full">
          <select
            class="block appearance-none w-full bg-gray-2 text-white border-gray-400 hover:border-gray-500 px-6 py-5 pr-8 rounded-xl shadow leading-tight focus:outline-none focus:shadow-outline"
            id="grid-state"
            onChange={handleChangeCoin}
          >
            {AVAILABLE_CHAINS.map((network) => (
              <option
                onClick={async () => {
                  try {
                    await wallet.walletClient?.addChain({
                      chain: unwrap(network.network),
                    })
                  } catch (error) {
                    console.log(error)
                  }

                  try {
                    await wallet.walletClient?.switchChain({
                      id: unwrap(network.network).id,
                    })
                  } catch (error) {
                    console.log(error)
                  }
                }}
              >
                {network.network.name}
              </option>
            ))}
          </select>
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-6 text-gray-700">
            <svg
              class="fill-current h-4 w-4"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
            >
              <path d="M10 12.586L2.929 5.515c-.39-.39-1.023-.39-1.414 0-.39.39-.39 1.023 0 1.414l8 8c.39.39 1.023.39 1.414 0l8-8c.39-.39.39-1.023 0-1.414-.39-.39-1.023-.39-1.414 0L10 12.586z" />
            </svg>
          </div>
        </div>
        <Show when={selectedAsset()}>
          <img
            src={joinPaths(base, wallet.selected_network.icon)}
            width="36px"
            height="36px"
          />
        </Show>
      </div>
      {/*  */}
      <div class="flex gap-4 items-center mb-2">
        <div class="relative inline-block w-full">
          <select
            class="block appearance-none w-full bg-gray-2 text-white border-gray-400 hover:border-gray-500 px-6 py-5 pr-8 rounded-xl shadow leading-tight focus:outline-none focus:shadow-outline"
            id="grid-state"
            onChange={handleChangeCoin}
          >
            {assetsList().map((asset) => (
              <option value={asset.id}>{asset.symbol}</option>
            ))}
          </select>
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-6 text-gray-700">
            <svg
              class="fill-current h-4 w-4"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
            >
              <path d="M10 12.586L2.929 5.515c-.39-.39-1.023-.39-1.414 0-.39.39-.39 1.023 0 1.414l8 8c.39.39 1.023.39 1.414 0l8-8c.39-.39.39-1.023 0-1.414-.39-.39-1.023-.39-1.414 0L10 12.586z" />
            </svg>
          </div>
        </div>
        <Show when={selectedAsset()}>
          <img
            src={joinPaths(
              base,
              '/gfx/asset_icons/' +
                (selectedAsset() as Asset).symbol.toLowerCase() +
                '.svg'
            )}
            width="36px"
            height="36px"
          />
        </Show>
      </div>
      <Show when={selectedAsset() && precision()}>
        <p class="text-sm text-gray-4 mb-6">
          {balance() != undefined
            ? `Your balance: ${format(
                ev(balance()!),
                formatTemplate(precision() ?? 3)
              )} ${selectedAsset()?.symbol}`
            : 'Your balance: '}
        </p>
      </Show>
      <Dynamic
        component={CreateProccessorDeposit(selectedAsset(), precision())}
      />
      {selectedAsset() && selectedAsset()!.name}
      {/*  */}
    </div>
  )
}
