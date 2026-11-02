<script setup lang="ts">
import { DownloadIcon, PlayIcon, RefreshCwIcon, ServerIcon, TrashIcon } from '@modrinth/assets'
import { ButtonStyled, defineMessages, injectNotificationManager, useVIntl } from '@modrinth/ui'
import { computed, onMounted, ref, watch } from 'vue'

import { useManagedInstances } from '@/composables/useManagedInstances'
import { useServerStatuses } from '@/composables/useServerStatuses'
import { remove as removeInstance, run } from '@/helpers/instance'
import { defaultInstanceIconUrl } from '@/helpers/instance-icons'
import type { ManagedInstanceRecord } from '@/helpers/managed'
import { managed_ensure_runnable } from '@/helpers/managed'

const { formatMessage } = useVIntl()
const { handleError } = injectNotificationManager()

const messages = defineMessages({
	title: { id: 'managed-instances.title', defaultMessage: 'Society game instances' },
	checking: {
		id: 'managed-instances.checking',
		defaultMessage: 'Checking the server for versions...',
	},
	updating: { id: 'managed-instances.updating', defaultMessage: 'Updating {name}' },
	progress: { id: 'managed-instances.progress', defaultMessage: '{done} of {total} done' },
	ready: { id: 'managed-instances.ready', defaultMessage: 'Up to date' },
	refresh: { id: 'managed-instances.refresh', defaultMessage: 'Check for updates' },
	play: { id: 'managed-instances.play', defaultMessage: 'Play' },
	playDownloads: {
		id: 'managed-instances.play-downloads',
		defaultMessage: 'Play (downloads first)',
	},
	starting: { id: 'managed-instances.starting', defaultMessage: 'Starting...' },
	notDownloaded: { id: 'managed-instances.not-downloaded', defaultMessage: 'Not downloaded yet' },
	remove: { id: 'managed-instances.remove', defaultMessage: 'Remove from this computer' },
	removeHint: {
		id: 'managed-instances.remove-hint',
		defaultMessage:
			'Removing it frees the disk space; the instance stays listed so you can download it again.',
	},
	badgeRequired: { id: 'managed-instances.badge-required', defaultMessage: 'Required' },
	badgeOptional: { id: 'managed-instances.badge-optional', defaultMessage: 'On demand' },
	playersOnline: {
		id: 'managed-instances.players-online',
		defaultMessage: '{online}/{max} online',
	},
	serverOffline: { id: 'managed-instances.server-offline', defaultMessage: 'Offline' },
	noInstances: {
		id: 'managed-instances.empty',
		defaultMessage: 'The server currently publishes no game instances.',
	},
	unconfigured: {
		id: 'managed-instances.unconfigured',
		defaultMessage:
			'No society instance manifest address is configured, so game instances cannot be fetched.',
	},
})

const {
	state,
	error,
	records,
	activeRecords,
	optionalActionsByServerId,
	specsById,
	isBusy,
	isConfigured,
	updatingName,
	manifestUrl,
	syncNow,
	ensureSynced,
	refresh,
	installOptional,
} = useManagedInstances()

const launchingId = ref<string | null>(null)
const removingId = ref<string | null>(null)

// Failures are reported as a notification instead of inline text: the panel is
// the first thing on the home page, and a sync that failed for a moment should
// not leave a permanent red line there.
watch(error, (value) => {
	if (value) handleError(value)
})

const statusText = computed(() => {
	if (state.value === 'syncing') return formatMessage(messages.checking)
	if (state.value === 'updating') {
		return formatMessage(messages.updating, { name: updatingName.value ?? '' })
	}
	if (state.value === 'ready') return formatMessage(messages.ready)
	return ''
})

function specFor(serverInstanceId: string) {
	return specsById.value.get(serverInstanceId) ?? null
}

/** An instance waiting for the player to download it, if this is one. */
function pendingAction(serverInstanceId: string) {
	return optionalActionsByServerId.value.get(serverInstanceId) ?? null
}

function versionLabel(serverInstanceId: string) {
	const spec = specFor(serverInstanceId)
	if (!spec) return ''
	const loader = spec.minecraft.loader
	if (loader && loader !== 'vanilla') {
		const loaderVersion = spec.minecraft.loader_version
		return `${spec.minecraft.game_version} · ${loader}${loaderVersion ? ` ${loaderVersion}` : ''}`
	}
	return spec.minecraft.game_version
}

function serverLabel(serverInstanceId: string) {
	const spec = specFor(serverInstanceId)
	const address = spec?.server?.address
	if (!address) return ''
	const port = spec?.server?.port
	return port && port !== 25565 ? `${address}:${port}` : address
}

function quickPlayAddress(serverInstanceId: string) {
	const address = specFor(serverInstanceId)?.server?.address
	if (!address) return null
	const port = specFor(serverInstanceId)?.server?.port
	return port && port !== 25565 ? `${address}:${port}` : address
}

/**
 * Addresses the active instances play on, sorted, so the online counts follow
 * whichever of the two sources arrives last: the records and their specs are
 * fetched independently, and an address read before its spec is known is missing
 * from that pass only.
 */
const serverAddresses = computed(() =>
	[
		...new Set(
			activeRecords.value
				.map((record) => quickPlayAddress(record.server_instance_id))
				.filter((address): address is string => Boolean(address)),
		),
	].sort(),
)

const { statusFor: serverStatus } = useServerStatuses(serverAddresses)

/**
 * Everything one card shows. Building it here keeps the template reading values
 * instead of repeating the same lookups per row, which also keeps the online
 * count to a single query per address per refresh.
 */
const cards = computed(() =>
	activeRecords.value.map((record) => {
		const id = record.server_instance_id
		const address = quickPlayAddress(id)
		const status = address ? serverStatus(address) : null

		return {
			record,
			version: versionLabel(id),
			server: serverLabel(id),
			pending: Boolean(pendingAction(id)),
			presence: !status
				? null
				: status.online
					? {
							text: formatMessage(messages.playersOnline, {
								online: status.players?.online ?? 0,
								max: status.players?.max ?? 0,
							}),
							offline: false,
						}
					: { text: formatMessage(messages.serverOffline), offline: true },
		}
	}),
)

async function launch(instanceId: string, serverInstanceId: string) {
	if (launchingId.value) return
	launchingId.value = instanceId
	try {
		// The server owns the revision, so re-read the manifest before launching
		// instead of trusting a record that may have been synced minutes ago. A
		// failed refresh (offline) leaves the previous records in place, so an
		// already installed instance can still start. The player asked, so an
		// instance that failed earlier is attempted again.
		await syncNow(true)
		// The launcher refuses stale instances in its own launch path as well;
		// checking here just turns that into an immediate, readable error.
		await managed_ensure_runnable(instanceId)
		await run(instanceId, quickPlayAddress(serverInstanceId))
		await refresh()
	} catch (launchFailure) {
		handleError(launchFailure)
	} finally {
		launchingId.value = null
	}
}

/**
 * Starts an instance, downloading it first when it is not on this computer.
 *
 * Optional instances are deliberately not fetched in the background, so the
 * player's press is what installs them; the launch follows once the install
 * settled and the record points at the new local instance.
 */
async function play(record: ManagedInstanceRecord) {
	const action = pendingAction(record.server_instance_id)

	if (!action) {
		await launch(record.instance_id, record.server_instance_id)
		return
	}

	await installOptional(action)
	await refresh()

	const updated = records.value.find(
		(candidate) => candidate.server_instance_id === record.server_instance_id,
	)
	if (!updated?.instance_id) return

	await launch(updated.instance_id, record.server_instance_id)
}

/** Frees an optional instance. The record stays, so it can be downloaded again. */
async function removeOptional(record: ManagedInstanceRecord) {
	if (!record.instance_id || removingId.value) return
	removingId.value = record.instance_id
	try {
		await removeInstance(record.instance_id)
		await refresh()
		// Re-reads the manifest so the instance is offered for download again.
		await syncNow(true)
	} catch (removalFailure) {
		handleError(removalFailure)
	} finally {
		removingId.value = null
	}
}

onMounted(() => {
	void ensureSynced()
})
</script>

<template>
	<section
		v-if="isConfigured"
		data-onboarding-id="managed-instances"
		class="rounded-[var(--radius-lg)] bg-bg-raised p-4 card-shadow"
	>
		<header class="mb-3 flex flex-wrap items-center justify-between gap-3">
			<h2 class="m-0 text-base font-bold text-contrast">
				{{ formatMessage(messages.title) }}
			</h2>
			<div class="flex items-center gap-3">
				<span v-if="statusText" class="text-xs text-secondary">{{ statusText }}</span>
				<ButtonStyled>
					<button :disabled="isBusy" @click="syncNow(true)">
						<RefreshCwIcon :class="{ 'animate-spin': isBusy }" />
						{{ formatMessage(messages.refresh) }}
					</button>
				</ButtonStyled>
			</div>
		</header>

		<p v-if="activeRecords.length === 0" class="m-0 text-sm text-secondary">
			{{ formatMessage(messages.noInstances) }}
		</p>

		<div v-else class="grid grid-cols-2 gap-x-2 gap-y-1">
			<div
				v-for="card in cards"
				:key="card.record.server_instance_id"
				class="grid min-w-0 grid-cols-[auto_1fr_auto] items-center gap-2 rounded-lg px-2 py-2 transition-colors hover:bg-button-bg"
			>
				<div class="relative shrink-0">
					<img
						:src="card.record.icon_url || defaultInstanceIconUrl"
						alt=""
						class="size-12 rounded-lg object-cover"
					/>
					<span
						v-if="card.presence"
						class="absolute -bottom-0.5 -right-0.5 size-2.5 rounded-full border-2 border-solid border-surface-2"
						:class="card.presence.offline ? 'bg-red' : 'bg-green'"
						aria-hidden="true"
					/>
				</div>

				<div class="flex min-w-0 flex-col gap-0.5">
					<span class="flex min-w-0 items-center gap-1.5">
						<span class="line-clamp-2 min-w-0 font-bold leading-normal text-contrast">
							{{ card.record.name }}
						</span>
						<span
							v-if="card.record.required"
							class="shrink-0 rounded-full bg-surface-3 px-1.5 py-px text-[10px] text-secondary"
						>
							{{ formatMessage(messages.badgeRequired) }}
						</span>
					</span>
					<span
						v-if="card.record.description"
						class="line-clamp-2 text-xs leading-snug text-secondary"
					>
						{{ card.record.description }}
					</span>
				</div>

				<div class="flex items-center gap-1">
					<ButtonStyled
						v-if="!card.record.required && card.record.instance_id && !card.pending"
						type="transparent"
						circular
					>
						<button
							v-tooltip="formatMessage(messages.removeHint)"
							:disabled="isBusy || removingId !== null"
							@click="removeOptional(card.record)"
						>
							<TrashIcon />
						</button>
					</ButtonStyled>

					<ButtonStyled :color="card.pending ? 'standard' : 'brand'" circular>
						<button
							v-tooltip="
								launchingId === card.record.instance_id
									? formatMessage(messages.starting)
									: card.pending
										? formatMessage(messages.playDownloads)
										: formatMessage(messages.play)
							"
							:disabled="isBusy || launchingId !== null || removingId !== null"
							@click="play(card.record)"
						>
							<DownloadIcon v-if="card.pending" />
							<PlayIcon v-else class="translate-x-[1px]" />
						</button>
					</ButtonStyled>
				</div>

				<div
					class="col-span-3 flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1 text-xs text-secondary"
				>
					<span v-if="card.version" class="truncate">{{ card.version }}</span>
					<span v-if="card.server" class="flex min-w-0 items-center gap-1">
						<ServerIcon class="size-3 shrink-0" />
						<span class="truncate">{{ card.server }}</span>
						<span
							v-if="card.presence"
							class="inline-flex shrink-0 items-center gap-1"
							:class="card.presence.offline ? 'text-red' : 'text-green'"
						>
							<span v-if="!card.presence.offline" class="size-1.5 rounded-full bg-green" />
							{{ card.presence.text }}
						</span>
					</span>
					<span v-if="card.pending" class="text-brand">
						{{ formatMessage(messages.notDownloaded) }}
					</span>
				</div>
			</div>
		</div>
	</section>

	<section
		v-else-if="manifestUrl === ''"
		class="rounded-[var(--radius-lg)] bg-bg-raised p-4 card-shadow"
	>
		<p class="m-0 text-sm text-secondary">{{ formatMessage(messages.unconfigured) }}</p>
	</section>
</template>
