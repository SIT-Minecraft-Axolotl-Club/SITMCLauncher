//! Endpoints of the SIT-Minecraft services this launcher build talks to.
//!
//! This launcher is distributed for the SIT-Minecraft society, which means there is exactly
//! one account provider and one server-provided instance catalog. Both are pinned
//! here rather than being user-configurable, so a player cannot point the launcher
//! at a different account system and every client agrees on where content comes
//! from.
//!
//! See `docs/launcher-protocol.md` for the contracts these endpoints implement.

/// Public site of the society's Blessing Skin deployment.
pub const SITE_URL: &str = "https://skin.sitmc.club";

/// Where a player creates the account the launcher accepts.
pub const REGISTER_URL: &str = "https://skin.sitmc.club/auth/register";

/// Where a player signs in with a browser.
pub const LOGIN_URL: &str = "https://skin.sitmc.club/auth/login";

/// Yggdrasil (authlib-injector compatible) API root of the skin site.
pub const YGGDRASIL_API_ROOT: &str = "https://skin.sitmc.club/api/yggdrasil";

/// Base URL of the Janus OpenID Connect provider behind the skin site.
pub const JANUS_BASE_URL: &str = "https://skin.sitmc.club/api/janus";

/// Human-readable name of the account site, used when the site does not report
/// one itself.
pub const SITE_LABEL: &str = "SIT-Minecraft";

/// The society's documentation, as shown in the launcher's Documentation page.
///
/// It is a Feishu wiki node, which is what lets the launcher walk the pages
/// below it for the page's own navigation.
pub const DOCS_URL: &str =
    "https://sit-minecraft.feishu.cn/wiki/BlvtwrH8diexAtklJg8cntwmnwc";

/// Client ID the site reserves for launchers (`shared_client_id` in the
/// OpenID provider metadata). The metadata is preferred when it announces one.
pub const OIDC_CLIENT_ID: &str = "11";

/// Scopes requested when starting a device authorization grant.
///
/// `Yggdrasil.PlayerProfiles.Select` makes the authorization page ask the player
/// which character to play as and binds the issued access token to it;
/// `Yggdrasil.Server.Join` is what lets that token join a Minecraft server. The
/// Yggdrasil Connect specification requires the joined server to be paired with
/// `Select`, and forbids pairing `Select` with `PlayerProfiles.Read`.
pub const OIDC_SCOPE: &str = "openid offline_access Yggdrasil.PlayerProfiles.Select Yggdrasil.Server.Join";

/// OAuth grant type of the device authorization grant (RFC 8628).
pub const OIDC_DEVICE_CODE_GRANT_TYPE: &str =
    "urn:ietf:params:oauth:grant-type:device_code";

/// Whether an API root addresses the society's skin site.
///
/// Compared after normalizing away a trailing slash so stored accounts created
/// by earlier builds keep working.
pub fn is_yggdrasil_api_root(api_root: &str) -> bool {
    api_root.trim().trim_end_matches('/') == YGGDRASIL_API_ROOT
}

/// Status endpoint of the society's server-query service. The address to look
/// up is appended as the path, so `play.sitmc.club` queries
/// `https://query.sitmc.club/play.sitmc.club`.
pub const SERVER_QUERY_BASE_URL: &str = "https://query.sitmc.club";

/// What the query service reports about one Minecraft server.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerStatus {
    pub online: bool,
    pub hostname: Option<String>,
    #[serde(default)]
    pub players: Option<ServerPlayerCounts>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub ping: Option<u64>,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerPlayerCounts {
    pub online: u64,
    pub max: u64,
}

/// Queries the society's server-query service for one address.
///
/// The response is returned even when the service itself reports the server
/// offline (`online: false`); only transport errors become `Err`.
pub async fn query_server(address: &str) -> crate::Result<ServerStatus> {
    let host = address.trim().trim_end_matches('/');
    let host = host.rsplit_once("://").map_or(host, |(_, rest)| rest);
    let url = format!("{SERVER_QUERY_BASE_URL}/{host}");

    let client = crate::util::fetch::build_proxied_client(
        &crate::util::proxy::ProxyConfig::default(),
    );
    let status = client
        .get(&url)
        .send()
        .await
        .map_err(|error| {
            crate::ErrorKind::LauncherError(format!(
                "Could not reach the server query service: {error}"
            ))
        })?
        .error_for_status()
        .map_err(|error| {
            crate::ErrorKind::LauncherError(format!(
                "The server query service rejected the request: {error}"
            ))
        })?;

    Ok(status.json().await.map_err(|error| {
        crate::ErrorKind::LauncherError(format!(
            "The server query service returned an unreadable response: {error}"
        ))
    })?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_the_sitmc_api_root() {        assert!(is_yggdrasil_api_root(YGGDRASIL_API_ROOT));
        assert!(is_yggdrasil_api_root(&format!("{YGGDRASIL_API_ROOT}/")));
        assert!(is_yggdrasil_api_root(&format!(" {YGGDRASIL_API_ROOT} ")));
    }

    #[test]
    fn rejects_other_api_roots() {
        assert!(!is_yggdrasil_api_root(
            "https://littleskin.cn/api/yggdrasil"
        ));
        assert!(!is_yggdrasil_api_root(""));
        assert!(!is_yggdrasil_api_root("https://skin.sitmc.club"));
    }
}
