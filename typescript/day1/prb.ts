const content = await Deno.readTextFile("../../inputs/d1");

const lines = content.split(["\n"]).map((x: string) => x.trim());
const values = lines.map((x: string) =>
  x.split(" ").map((x: string) => x.trim())
)
  .flat()
  .filter((x: string) => x != "");

const left: number[] = [];
const right: number[] = [];
values.forEach((x: string, index: number) => {
  if (index % 2 == 0) {
    right.push(parseInt(x));
  } else {
    left.push(parseInt(x));
  }
});

left.sort();
right.sort();

function part1(left: number[], right: number[]): number {
  let count = 0;
  left.forEach((x: number, index: number) => {
    count += Math.abs(x - right[index]);
  });
  return count;
}
function part2(left: number[], right: number[]): number {
  let acc = 0;
  left.forEach((x: number) => {
    acc += x * right.filter((v: number) => x == v).length;
  });
  return acc;
}

console.log(`Part1: ${part1(left, right)}`);
console.log(`Part2: ${part2(left, right)}`);
