//
// Main Entry Point
//


let main_div = <HTMLDivElement>document.getElementById("main");

let label  = document.createElement("label");
let button = document.createElement("button");

button.textContent = "Press me!";

button.onclick = (): void => {
    if (label.textContent == "") {
        label.textContent = "I was just clicked!";
    } else {
        label.textContent = "";
    }
};

main_div.appendChild(label);
main_div.appendChild(button);
