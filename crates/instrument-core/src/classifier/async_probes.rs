use crate::scpi::AsyncScpiSession;
use std::time::Duration;

/// Returns true when any probe command succeeds on the async session.
pub async fn probe_any_async(
    session: &mut AsyncScpiSession,
    commands: &[&str],
    timeout: Duration,
) -> bool {
    for cmd in commands {
        if session.query_with_timeout(cmd, timeout).await.is_ok() {
            return true;
        }
    }
    false
}
