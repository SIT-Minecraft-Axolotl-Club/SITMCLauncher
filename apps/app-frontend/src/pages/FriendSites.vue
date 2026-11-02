<script setup lang="ts">
import {
	DownloadIcon,
	GlobeIcon,
	PlayIcon,
	RefreshCwIcon,
	ServerIcon,
	TrashIcon,
} from '@modrinth/assets'
import { ButtonStyled, defineMessages, injectNotificationManager, useVIntl } from '@modrinth/ui'
import { computed, onMounted, ref } from 'vue'

import { useServerStatuses } from '@/composables/useServerStatuses'
import { SitmcConfig } from '@/config'
import type { FriendInstance, FriendsDocument, FriendSite } from '@/helpers/friend-sites'
import {
	friends_fetch,
	friendSitesWithInstances,
	installedInstancesByName,
	installFriendInstance,
	isInstallable,
	quickPlayAddress,
	uninstallFriendInstance,
} from '@/helpers/friend-sites'
import { run } from '@/helpers/instance'
import { defaultInstanceIconUrl } from '@/helpers/instance-icons'
import type { SitmcServerStatus } from '@/helpers/server-query'

const { formatMessage } = useVIntl()
const { handleError } = injectNotificationManager()

const messages = defineMessages({
	title: { id: 'friend-sites.title', defaultMessage: 'Instances from friend sites' },
	summary: {
		id: 'friend-sites.summary',
		defaultMessage: '{sites} sites · {instances} instances',
	},
	refresh: { id: 'friend-sites.refresh', defaultMessage: 'Refresh' },
	loading: {
		id: 'friend-sites.loading',
		defaultMessage: 'Looking for instances from friend sites...',
	},
	empty: {
		id: 'friend-sites.empty',
		defaultMessage: 'No friend site publishes an instance right now.',
	},
	failed: {
		id: 'friend-sites.failed',
		defaultMessage: 'The friend-site list could not be loaded.',
	},
	retry: { id: 'friend-sites.retry', defaultMessage: 'Retry' },
	install: { id: 'friend-sites.install', defaultMessage: 'Install' },
	installing: { id: 'friend-sites.installing', defaultMessage: 'Installing...' },
	installed: { id: 'friend-sites.installed', defaultMessage: 'Installed' },
	play: { id: 'friend-sites.play', defaultMessage: 'Play' },
	starting: { id: 'friend-sites.starting', defaultMessage: 'Starting...' },
	remove: { id: 'friend-sites.remove', defaultMessage: 'Remove from this computer' },
	removeHint: {
		id: 'friend-sites.remove-hint',
		defaultMessage:
			'Removing it frees the disk space; the instance stays listed so you can install it again.',
	},
	unavailable: { id: 'friend-sites.unavailable', defaultMessage: 'Not available for download' },
	stale: {
		id: 'friend-sites.stale',
		defaultMessage: 'Not refreshed just now; this list may be out of date.',
	},
	visit: { id: 'friend-sites.visit', defaultMessage: 'Visit the site' },
	playersOnline: {
		id: 'friend-sites.players-online',
		defaultMessage: '{online}/{max} online',
	},
	serverOffline: { id: 'friend-sites.server-offline', defaultMessage: 'Offline' },
})

/** Tauri rejects a command with a string or a plain object, not always an Error. */
function describeError(error: unknown): string {
	if (error instanceof Error) return error.message
	if (typeof error === 'string') return error
	if (error && typeof error === 'object') {
		const message = (error as { message?: unknown }).message
		if (typeof message === 'string' && message.length > 0) return message
	}
	return String(error)
}

const urls = [SitmcConfig.friendsUrl, SitmcConfig.friendsInteropUrl].filter(Boolean)
const isConfigured = urls.length > 0

const loading = ref(false)
const error = ref<string | null>(null)
const catalog = ref<FriendsDocument | null>(null)
const installed = ref<Map<string, string>>(new Map())
const installingKey = ref<string | null>(null)
const launchingId = ref<string | null>(null)
const removingId = ref<string | null>(null)

const sites = computed<FriendSite[]>(() => friendSitesWithInstances(catalog.value))

const instanceCount = computed(() =>
	sites.value.reduce((total, site) => total + site.instances.length, 0),
)

async function load(): Promise<void> {
	if (!isConfigured || loading.value) return

	loading.value = true
	error.value = null

	try {
		catalog.value = await friends_fetch(urls)
		installed.value = await installedInstancesByName()
	} catch (loadError) {
		error.value = describeError(loadError)
	} finally {
		loading.value = false
	}
}

function instanceKey(site: FriendSite, instance: FriendInstance): string {
	return `${site.site_id}:${instance.id}`
}

/** The local copy of this entry, when it is installed. */
function installedId(instance: FriendInstance): string | null {
	return installed.value.get(instance.name.trim()) ?? null
}

function loaderLabel(instance: FriendInstance): string {
	const minecraft = instance.minecraft
	if (!minecraft) return ''
	const loader = minecraft.loader?.trim()
	if (!loader) return ''
	const version = minecraft.loader_version ? ` ${minecraft.loader_version}` : ''
	return `${loader}${version}`
}

function serverLabel(instance: FriendInstance): string {
	const server = instance.server
	if (!server?.address) return ''
	return server.port && server.port !== 25565 ? `${server.address}:${server.port}` : server.address
}

/** Every address the listed instances play on, so each is queried once. */
const serverAddresses = computed(() =>
	[
		...new Set(
			sites.value
				.flatMap((site) => site.instances)
				.map((instance) => serverLabel(instance))
				.filter((address) => address.length > 0),
		),
	].sort(),
)

const { statusFor } = useServerStatuses(serverAddresses)

function serverStatus(instance: FriendInstance): SitmcServerStatus | null {
	const address = serverLabel(instance)
	return address ? statusFor(address) : null
}

async function install(site: FriendSite, instance: FriendInstance): Promise<void> {
	const key = instanceKey(site, instance)
	if (installingKey.value) return

	installingKey.value = key
	try {
		await installFriendInstance(instance)
		installed.value = await installedInstancesByName()
	} catch (installError) {
		handleError(installError)
	} finally {
		installingKey.value = null
	}
}

/** Starts an installed entry straight from this page. */
async function launch(instance: FriendInstance): Promise<void> {
	const id = installedId(instance)
	if (!id || launchingId.value) return

	launchingId.value = id
	try {
		await run(id, quickPlayAddress(instance))
	} catch (launchError) {
		handleError(launchError)
	} finally {
		launchingId.value = null
	}
}

/** Frees an installed entry. It stays listed, so it can be installed again. */
async function removeInstalled(instance: FriendInstance): Promise<void> {
	const id = installedId(instance)
	if (!id || removingId.value) return

	removingId.value = id
	try {
		await uninstallFriendInstance(id)
		installed.value = await installedInstancesByName()
	} catch (removeError) {
		handleError(removeError)
	} finally {
		removingId.value = null
	}
}

onMounted(() => {
	void load()
})
</script>

<template>
	<div class="flex h-full min-h-0 flex-col gap-4 overflow-y-auto p-6">
		<header class="flex flex-wrap items-center justify-between gap-3">
			<h1 class="m-0 text-lg font-semibold text-contrast">
				{{ formatMessage(messages.title) }}
			</h1>
			<div class="flex items-center gap-3">
				<span v-if="instanceCount > 0" class="text-xs text-secondary">
					{{
						formatMessage(messages.summary, {
							sites: sites.length,
							instances: instanceCount,
						})
					}}
				</span>
				<ButtonStyled>
					<button :disabled="loading" @click="load">
						<RefreshCwIcon :class="{ 'animate-spin': loading }" />
						{{ formatMessage(messages.refresh) }}
					</button>
				</ButtonStyled>
			</div>
		</header>

		<section
			v-if="error"
			class="flex flex-wrap items-center gap-3 rounded-[var(--radius-lg)] bg-bg-raised p-4 card-shadow"
		>
			<p class="m-0 flex-1 text-sm text-red">{{ formatMessage(messages.failed) }} {{ error }}</p>
			<ButtonStyled size="small">
				<button :disabled="loading" @click="load">
					{{ formatMessage(messages.retry) }}
				</button>
			</ButtonStyled>
		</section>

		<section
			v-else-if="loading && sites.length === 0"
			class="flex items-center gap-3 rounded-[var(--radius-lg)] bg-bg-raised p-4 card-shadow"
		>
			<DownloadIcon class="size-5 shrink-0 text-brand" />
			<p class="m-0 text-sm text-secondary">{{ formatMessage(messages.loading) }}</p>
		</section>

		<section
			v-else-if="sites.length === 0"
			class="rounded-[var(--radius-lg)] bg-bg-raised p-4 card-shadow"
		>
			<p class="m-0 text-sm text-secondary">{{ formatMessage(messages.empty) }}</p>
		</section>

		<template v-else>
			<section
				v-for="site in sites"
				:key="site.site_id"
				class="rounded-[var(--radius-lg)] bg-bg-raised p-4 card-shadow"
			>
				<header class="mb-3 flex flex-wrap items-center justify-between gap-3">
					<div class="flex min-w-0 flex-col">
						<h2 class="m-0 truncate text-base font-bold text-contrast">
							{{ site.name }}
						</h2>
						<span v-if="site.description" class="truncate text-xs text-secondary">
							{{ site.description }}
						</span>
					</div>
					<div class="flex items-center gap-3">
						<a
							v-if="site.domain"
							v-tooltip="formatMessage(messages.visit)"
							:href="`https://${site.domain}`"
							target="_blank"
							rel="noreferrer"
							class="flex items-center gap-1 text-xs text-secondary transition-colors hover:text-contrast"
						>
							<GlobeIcon class="size-3.5" />
							{{ site.domain }}
						</a>
						<span
							v-if="site.stale"
							v-tooltip="formatMessage(messages.stale)"
							class="rounded-full bg-surface-3 px-2 py-0.5 text-[11px] text-secondary"
						>
							!
						</span>
					</div>
				</header>

				<div class="grid grid-cols-2 gap-x-2 gap-y-1">
					<div
						v-for="instance in site.instances"
						:key="instance.id"
						class="grid min-w-0 grid-cols-[auto_1fr_auto] items-center gap-2 rounded-lg px-2 py-2 transition-colors hover:bg-button-bg"
					>
						<div class="relative shrink-0">
							<img
								:src="instance.icon_url || defaultInstanceIconUrl"
								alt=""
								class="size-12 rounded-lg object-cover"
							/>
							<span
								v-if="serverStatus(instance)"
								class="absolute -bottom-0.5 -right-0.5 size-2.5 rounded-full border-2 border-solid border-surface-2"
								:class="serverStatus(instance)!.online ? 'bg-green' : 'bg-red'"
								aria-hidden="true"
							/>
						</div>

						<div class="flex min-w-0 flex-col gap-0.5">
							<span class="line-clamp-2 min-w-0 font-bold leading-normal text-contrast">
								{{ instance.name }}
							</span>
							<span
								v-if="instance.description"
								class="line-clamp-2 text-xs leading-snug text-secondary"
							>
								{{ instance.description }}
							</span>
							<span v-else-if="site.name" class="truncate text-[11px] text-secondary">
								{{ site.name }}
							</span>
						</div>

						<div class="flex items-center gap-1">
							<template v-if="installedId(instance)">
								<ButtonStyled type="transparent" circular>
									<button
										v-tooltip="formatMessage(messages.removeHint)"
										:disabled="launchingId !== null || removingId !== null"
										@click="removeInstalled(instance)"
									>
										<TrashIcon />
									</button>
								</ButtonStyled>
								<ButtonStyled color="brand" circular>
									<button
										v-tooltip="formatMessage(messages.play)"
										:disabled="launchingId !== null || removingId !== null"
										@click="launch(instance)"
									>
										<PlayIcon class="translate-x-[1px]" />
									</button>
								</ButtonStyled>
							</template>
							<ButtonStyled v-else-if="isInstallable(instance)" color="brand" circular>
								<button
									v-tooltip="formatMessage(messages.install)"
									:disabled="installingKey !== null"
									@click="install(site, instance)"
								>
									<DownloadIcon />
								</button>
							</ButtonStyled>
							<span v-else class="text-[11px] text-secondary">
								{{ formatMessage(messages.unavailable) }}
							</span>
						</div>

						<div
							class="col-span-3 flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1 text-xs text-secondary"
						>
							<span v-if="instance.minecraft?.game_version" class="truncate">
								{{ instance.minecraft.game_version }}
							</span>
							<span v-if="loaderLabel(instance)" class="truncate capitalize">
								{{ loaderLabel(instance) }}
							</span>
							<span v-if="serverLabel(instance)" class="flex min-w-0 items-center gap-1">
								<ServerIcon class="size-3 shrink-0" />
								<span class="truncate">{{ serverLabel(instance) }}</span>
								<template v-if="serverStatus(instance)">
									<span
										v-if="serverStatus(instance)!.online"
										class="flex shrink-0 items-center gap-1 text-green"
									>
										<span class="size-1.5 rounded-full bg-green" />
										{{
											formatMessage(messages.playersOnline, {
												online: serverStatus(instance)!.players?.online ?? 0,
												max: serverStatus(instance)!.players?.max ?? 0,
											})
										}}
									</span>
									<span v-else class="shrink-0 text-red">
										{{ formatMessage(messages.serverOffline) }}
									</span>
								</template>
							</span>
						</div>
					</div>
				</div>
			</section>
		</template>
	</div>
</template>
