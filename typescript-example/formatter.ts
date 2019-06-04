function reFormat(reference) {
    /* 
    Bible verse references are traditionally formatted: <BookName> Chapter:StartVerse - EndVerse
    Example: Genesis 1:1-9 (Genesis chapter 1, verses 1 through 9)
    With the major edge case being: 
        1-4,6 (Continuous range and then discrete verse i.e. verses 1 through 4, and 6)
    This function takes a reference in the form <BookName> Chapter:V1,V2,..,VN and formats it
    in the traditional way.
    */
    let bookChapter = reference.slice(0, reference.indexOf(":")) // Example: Genesis 1:1,2,3,4 => Genesis 1
    let verses = reference.slice(reference.indexOf(":") + 1, reference.length) // Example: Genesis 1:1,2,3,4 => 1,2,3,4
    let digits = verses.split(",") // Creates array out of the verses
    for (let index in digits) {
        digits[index] = Number(digits[index]) // Iterate through the list and turn each element into an integer
    }
    digits.sort(function(a, b){return a-b}) // Ensures all verse numbers are in ascending order
    var lo = digits[0]
    var hi = -1
    let ranges = [] // Container for formatted verse segments
    for (let index in digits) {
        
        if (digits[Number(index) + 1] - digits[Number(index)] == 1) { // If elements are adjacent on the number line...
            hi = digits[Number(index) + 1] // Increase the variable representing the end of a continuous range (like 1-4)
        }

        else if (digits[Number(index) + 1] - digits[Number(index)] != 1) { // If two elements of the array are not adjacent...
            // The two blocks below check to see if the failiure was the end of a range or a discrete jump
            if (hi == -1) { 
                let strlo = String(lo)
                ranges.push(strlo)
                lo = digits[Number(index) + 1]
                hi = -1     
            }

            else if (hi != -1) {
                let strlo = String(lo)
                let strhi = String(hi)
                let range = strlo.concat("-", strhi)
                ranges.push(range)
                lo = digits[Number(index) + 1]
                hi = -1
            }
        }

        else if (Number(index) == (digits.length - 1) ) { // If you're at the end of the array perform a failure check
            
            if (hi == -1) {
                ranges.push(String(lo))
                lo = digits[Number(index) + 1]
                hi = -1
                break // Don't bother checking the last element because array[index + 1] is out of range
            }

            else if (hi != -1) {
                let range = String(lo).concat("-", String(hi))
                ranges.push(range)
                lo = digits[Number(index) + 1]
                hi = -1   
                break // Don't bother checking the last element because array[index + 1] is out of range
            }
        }
    }
    let formattedDigits = ranges.join(",")
    let completeReference = bookChapter.concat(":", formattedDigits)
    return completeReference
}
