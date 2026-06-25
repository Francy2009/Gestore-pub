#!/bin/bash
# postrm script for The Club .deb package
# Preserves user data by default. Data is only wiped on explicit "purge",
# never on "remove" (a "remove" + "install" is how many Ubuntu update flows
# perform an upgrade, so wiping on "remove" would silently destroy the user's
# desktop-db.json during an update). Manual data removal is available in-app
# via Settings → "Rimuovi dati locali" (cleanup_app_data Tauri command).

set -e

# On upgrade/failed-upgrade or plain remove: do NOT touch user data.
if [ "$1" = "upgrade" ] || [ "$1" = "failed-upgrade" ] || [ "$1" = "remove" ]; then
    exit 0
fi

# Only on full "purge" remove the app data directories.
if [ "$1" = "purge" ]; then
    if [ -d "$HOME/.local/share/com.the.club" ]; then
        rm -rf "$HOME/.local/share/com.the.club"
    fi
    if [ -d "$HOME/.config/com.the.club" ]; then
        rm -rf "$HOME/.config/com.the.club"
    fi
fi

exit 0