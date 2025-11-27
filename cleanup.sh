DAY_NUM=$(ls src/day*.rs | sed -E 's/src\/day([0-9]+)\.rs/\1/' | sort -n | tail -1)
# in the lines below the line "mod tests {", remove all lines that start with "//"
sed -i '/^mod tests {/,/^}/ {/^[[:space:]]*\/\/.*$/d}' src/day${DAY_NUM}.rs