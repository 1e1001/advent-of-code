source ../.env
get_tz() { TZ="America/Detroit" date +%"$1"; }
if [ -z "$DAY" -o "$DAY" = "_" ]; then DAY="$(get_tz d)"; fi
DAY=$((1$DAY - 100))
