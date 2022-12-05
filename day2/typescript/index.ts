import { readFileSync } from "fs";

const file = String(readFileSync("../input.txt")).split("\n")

const parse =  (i:string)=>{
    switch (i){
        case "A":{return 1}
        case "B":{return 2}
        case "C":{return 3}
        case "X":{return 1}
        case "Y":{return 2}
        case "Z":{return 3}
        default:{return 100000000}
    }
}
const score = (x:number,y:number)=>{
    if (x==y){
        return x+3
    }else if(x==y+1){
        return x+6
    }else if(x==y-2){
        return x+6
    }else{
        return x
    }
}
const guess = (x:number,y:number)=>{
    if (x==2){
        return score(y,y)
    }if (x==3){
        if (y==3){
            return score(1,3)
        }else{
            return score(y+1,y)
        }
    }else{
        if (y==1){
            return score(3,1)
        }else{
            return score(y-1,y)
        }
    }
}
const main = (input:string[])=>{
    const idk = input.map(e=>e.split(" ").map(i=>parse(i))).map(j=>score(j[1],j[0])).reduce((a,b)=>a+b,0)
    const idk2 = input.map(e=>e.split(" ").map(i=>parse(i))).map(j=>guess(j[1],j[0])).reduce((a,b)=>a+b,0)
    console.log("Stage 1 = ",idk)
    console.log("Stage 2 = ",idk2)
}

main(file)