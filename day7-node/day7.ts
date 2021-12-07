let input: string = '16,1,2,0,4,2,7,1,2,14';

let input_split = input.split(',').map(item => parseInt(item));

let min_pos = Math.min(...input_split);
let max_pos = Math.max(...input_split);

const reducer = (previousValue, currentValue) => previousValue + currentValue;

let best_fuel: number | null = null;
for (let pos = min_pos; pos <= max_pos; pos++) {
    let fuel = input_split.map(crab_pos => Math.abs(crab_pos - pos)).reduce(reducer);
    if (best_fuel === null || fuel < best_fuel) {
        best_fuel = fuel;
    }
}
console.log("Best fuel is " + best_fuel);