const ORBITAL_BUS_MAX_LEVEL = "orbital-bus-max-level";
const ORBITAL_BUS_LEVEL = "orbital-bus-level";

function main(){
    let maxLevelStr = localStorage.getItem(ORBITAL_BUS_MAX_LEVEL);
    if(maxLevelStr === null){
        maxLevelStr = "1";
        localStorage.setItem(ORBITAL_BUS_MAX_LEVEL, maxLevelStr);
    }
    const maxLevel = parseInt(maxLevelStr);
    const grid = document.getElementById("grid");
    for(let i=0;i<grid.children.length;i++){
        const child = grid.children[i];
        if(child.dataset){
            if(parseInt(child.dataset.level) <= maxLevel){
                child.className = "enabled";
                child.addEventListener("click", () => {
                    localStorage.setItem(ORBITAL_BUS_LEVEL, child.dataset.level);
                    window.location.href = "game.html";
                });
            }
        }
    }
    
}
main();