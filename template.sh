DAY_NUM=$1
if [ -z "$DAY_NUM" ]; then
    DAY_NUM=$(date +%-d)
    echo "No day number provided, using today's date: $DAY_NUM"
fi
DAY_FILE="src/day${DAY_NUM}.rs"
LIB_FILE="src/lib.rs"
TEMPLATE_FILE="src/day.rs"
cp $TEMPLATE_FILE $DAY_FILE
sed -i "s/day__/day${DAY_NUM}/g" $DAY_FILE
grep -q "pub mod day${DAY_NUM};" $LIB_FILE || sed -i "$(( $(wc -l < $LIB_FILE) - 3 ))i pub mod day${DAY_NUM};" $LIB_FILE
echo "https://adventofcode.com/2016/day/${DAY_NUM}"
code $DAY_FILE