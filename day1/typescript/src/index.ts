import {readFileSync} from 'fs'

const data = String(readFileSync("../input.txt"))


function calories(data:string){
    const elves = data.split("\n\n")
    return elves.map(e=>{
        return e.split("\n").reduce((a,b)=>Number(a)+Number(b),0)
    })
}
function max_value(data:number[]){
    return Math.max(...data)
}
const three_highest = (data:number[])=>{
    let vect:number[]=[]
    for (let i=0; i<3 ;i++){
        const highest = Math.max(...data)
        vect.push(highest)
        data.splice(data.indexOf(highest),1)
    }
    return vect.reduce((a,b)=>a+b,0)
}
let list_calories = calories(data)
console.log("stage 1 = "+max_value(list_calories))
console.log("stage 2 = "+three_highest(list_calories))