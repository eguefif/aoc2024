const content = await Deno.readTextFile("../../inputs/d2");

const tmp = content.split("\n").filter((line: string) => line.trim());
let lines = tmp.map((value: string) =>
  value.split(" ").map((value: string) => Number(value))
);

const is_safe = (x: number[]): boolean => {
  let n = x.length;
  let diffs: number[] = [];
  for (let i: number = 1; i < n; i++) {
    diffs.push(x[i] - x[i - 1]);
  }
  const asc = diffs.every((value) => value >= 1 && value <= 3);
  const desc = diffs.every((value: number): boolean =>
    value >= -3 && value <= -1
  );
  return (asc || desc) && x.length > 0;
};

const ans = lines.filter(is_safe);

const check_part2 = (line: number[]): boolean => {
  if (is_safe(line)) return true;

  let len = line.length;
  for (let x: number = 0; x <= len; x++) {
    if (is_safe(line.filter((_e, i) => i != x))) return true;
  }

  return false;
};

const ans2 = lines.filter(check_part2);

console.log(`Part1: ${ans.length}`);
console.log(`Part2: ${ans2.length}`);
