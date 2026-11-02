import { invoke } from '@tauri-apps/api/core'

/**
 * The society's server-query service reports one Minecraft server's status as
 * JSON at `https://query.sitmc.club/<address>`, so a card can show how many
 * players are online without pinging the server itself.
 */
export interface SitmcServerStatus {
	online: boolean
	hostname?: string
	players?: { online: number; max: number }
	version?: string
	ping?: number
	error?: string
}

/**
 * Queries one address. An unreachable service resolves to `null` rather than
 * throwing, so a card simply shows nothing instead of an error.
 */
export async function querySitmcServer(address: string): Promise<SitmcServerStatus | null> {
	const host = address
		.trim()
		.replace(/^\w+:\/\//, '')
		.replace(/\/$/, '')
	if (!host) return null

	return await invoke<SitmcServerStatus>('sitmc_query_server', {
		address: host,
	}).catch(() => null)
}
