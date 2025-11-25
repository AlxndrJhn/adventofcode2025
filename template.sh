DAY_NUM=$1
YEAR=2017
if [ -z "$DAY_NUM" ]; then
    if [ -z "$(ls src/day0*.rs 2>/dev/null)" ]; then
        # if there are no dayXX files yet, start with day 1
        DAY_NUM=1
    else
        # get the highest numbered day file
        DAY_NUM=$(ls src/day*.rs | sed -E 's/src\/day([0-9]+)\.rs/\1/' | sort -n | tail -1)
        DAY_NUM=$((DAY_NUM + 1))
    fi
fi
cargo aoc input -d $DAY_NUM -y $YEAR
DAY_FILE="src/day${DAY_NUM}.rs"
LIB_FILE="src/lib.rs"
TEMPLATE_FILE="src/day.rs"
cp $TEMPLATE_FILE $DAY_FILE
sed -i "s/day__/day${DAY_NUM}/g" $DAY_FILE
sed -i 's/^pub mod/\/\/ pub mod/' $LIB_FILE
grep -q "pub mod day${DAY_NUM};" $LIB_FILE || sed -i "$(( $(wc -l < $LIB_FILE) - 3 ))i pub mod day${DAY_NUM};" $LIB_FILE
cargo fmt
echo "https://adventofcode.com/${YEAR}/day/${DAY_NUM}"
code $DAY_FILE