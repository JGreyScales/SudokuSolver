const { invoke } = window.__TAURI__.tauri


function beginSolve(){

    passableArray = []
    mainTable = document.getElementById("MainTable").children[0].children
    tableWidth = mainTable.length


    for (let row = 0; row < tableWidth; row++) {
        tempArray = []
        for (let column = 0; column < tableWidth; column++){
            value = mainTable[row].children[column].firstChild.value
            if (value === ""){value = 0}
            tempArray.push(Number(value))
        }
        passableArray.push(tempArray)
    }

    invoke('solve', { dataTable: passableArray })
    // `invoke` returns a Promise
    .then((response) => {
        console.log(response)
    })
}