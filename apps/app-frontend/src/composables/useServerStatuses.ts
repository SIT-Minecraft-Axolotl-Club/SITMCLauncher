import { onUnmounted, type Ref,ref, watch } from 'vue'

import type { SitmcServerStatus } from '@/helpers/server-query'
import { querySitmcServer } from '@/helpers/server-query'

/**
 * How often the listed servers are asked again. The query service itself caches
 * upstream, so a minute keeps the count reasonably fresh without making the
 * launcher a load source.
 */
const REFRESH_INTERVAL_MS = 60_000

/**
 * Keeps the online counts of the servers behind a list of addresses.
 *
 * Addresses arrive from data that loads in pieces, so the hook watches the whole
 * list rather than a single load event. Polling only runs while a component is
 * mounted and stops as soon as the component unmounts.
 */
export function useServerStatuses(addresses: Ref<string[]>) {
	const statuses = ref<Record<string, SitmcServerStatus | null>>({})

	async function refresh(): Promise<void> {
		const list = addresses.value
		if (list.length === 0) {
			statuses.value = {}
			return
		}

		await Promise.all(
			list.map(async (address) => {
				statuses.value[address] = await querySitmcServer(address)
			}),
		)
	}

	watch(addresses, refresh, { immediate: true })

	const timer = setInterval(refresh, REFRESH_INTERVAL_MS)
	onUnmounted(() => clearInterval(timer))

	function statusFor(address: string): SitmcServerStatus | null {
		return address ? (statuses.value[address] ?? null) : null
	}

	return { statuses, statusFor, refresh }
}
