var desiredSize = 6

// / Take the default values from the inputs and create the table for that
HandleSize(document.getElementsByClassName("Size")[0])


// Create a fresh div to be inserted into the table on request
function GenerateBlankInput(element){
    newInput = document.createElement("input")
    newInput.max = element.max
    newInput.min = 1
    newInput.type = "number"
    newInput.className = "numberInput"
    newColumn = document.createElement("td")
    newColumn.append(newInput)
    return newColumn
}


// Deletes the last child of the div n times
function DelLastChildren(element, x){
    for (let index = 0; index < x; index++) {element.lastElementChild.remove()}
}


// Main resize function
function HandleSize(element) {



      // error prone sizes, 3, 5, 7, 11, 13, 17

    // Ensure bounds are respected
    let sizes = document.getElementsByClassName("Size")
    if (element.value < 2){ element.value = 2 }
    else if (element.value > Number(element.max)){ element.value = Number(element.max) }
    else{
        // Handle illegal sized Sudoku boards for tradtional box sizes
        switch (Number(element.value)) {
            case 3: element.value = 2; break;
            case 5: element.value = 4; break;
            case 7: element.value = 6; break;
            case 11: element.value = 10; break;
            case 13: element.value = 12; break;
            case 17: element.value = 16; break;
        }
    }




    // Ensure that both size box are equal
    // Too costly to check which box is context; its easier to just override them both
    desiredSize = element.value
    sizes[0].value = element.value
    sizes[1].value = element.value


    // Extract table
    let tableContainer = document.getElementById("MainTable").children[0]
    let table = tableContainer.children


    // Handle Height
    currentHeight = table.length
    if (currentHeight < desiredSize){
        amountToGrow = desiredSize - currentHeight
        for (let index = 0; index < amountToGrow; index++) {
            tableContainer.append(document.createElement("tr"))
        }
    } else if (currentHeight > desiredSize){
        amountToShrink = currentHeight - desiredSize
        DelLastChildren(tableContainer, amountToShrink)
    }
      
    
    // Handle Width
    for (const childRow of table) {
        currentLength = childRow.childNodes.length
            // If the table is smaller then the requested size; grow table
            if (currentLength < desiredSize){
                amountToGrow = desiredSize - currentLength
                for (let index = 0; index < amountToGrow; index++) {
                    childRow.append(GenerateBlankInput(element))
                }
            // If the table is larger than the requested size; shrink the table
            } else if (currentLength > desiredSize){
                amountToShrink = currentLength - desiredSize
                DelLastChildren(childRow, amountToShrink)
            }
        }
}