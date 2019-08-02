use std::Vec;

struct CheckerStruct{
    is_simple: bool,
    puzzle_complexity: bool,
    dict_complexity: bool,
}

struct OutputStruct{
    current_word: String::new(),
    words_matched: u8,
    dict_entries_visited: u32,
    word_length: i8,
    cells_visited: u64,
}

struct VectorStruct{
    //simple dictionary vector for read in
    simple_dictionary: Vec<SimpleResult> = Vec::new(),
    //puzzle grid
    simple_puzzle_grid: Vec<Vec<Char>> = Vec::new(),
    //this vector holds the modifiers for the loop
    modifier_vector: Vec<Vec<u8>> = vec![],
    //vector to hold the found words
    found_words: Vec<String::new()> = vec![]
}



struct SimpleResult {
    x: u8,
    y: u8,
    is_found: bool,
    value: String::new()
}


fn WordSearch() {
    //This constant holds the correct file name for the puzzle (e.g., wordsearch_grid.txt)
    const puzzle_name:String::new() = "wordsearch_grid.txt";
    //This constant holds the correct file name for the puzzle (e.g., wordsearch_grid.txt)
    const dictionary_name:String::new() = "dictionary.txt";

    let output_file: char;
}

fn main() {

}


//generic func to loop as long as i < word length
// && whilst x&y are in the bounds of the grid
fn get_direction_line (){
    //current word being searched
    wordLength = currentWord.length();

    const width: u8 = 9;

    let mut concatenate: String::new() = "";

    for i in 0..(len && x >= 0 && y >= 0 && x < width && y < width; x += xModifier,
    y += yModifier) {
        concatenate += simple_puzzle_grid[y][x];
    }
    //c++ code for ref
    /*
    for (int i = 0; i < len && x >= 0 && y >= 0 && x < width && y < width; i++,
        //x & y are modified using the modifierArray
        x += xModifier,
        y += yModifier)
    {
    conc += simplePuzzleGrid[y][x];
    }
    */
}

// This method will red the puzzle and store the letters in the simple grid data structure
fn read_simple_puzzle() {
    //local 2d vector to hold values until sent to simplePuzzleGrid
    let mut a: Vec<Vec<char>> = vec![];

    //initialise the row and column for the grid
    let mut column: u8 = 0;
    let mut row: u8 = 0;
    let mut gridSize: u8 = 0;

    ifstream.puzzleFile;
    puzzleFile.open(puzzleName);
    if (puzzleFile.is_open())
        {
            puzzleFile >> gridSize;
            row = gridSize;
            column = gridSize;

            //go along the rows and columns reading in the values from the .txt
            for (int columnLength = 0; columnLength < column; columnLength++)
            {
            vector<char> m_row;
            for (int rowLenght = 0; rowLenght < row; rowLenght++)
            {
            char c;
            puzzleFile >> c;
            m_row.push_back(c);
            //puzzleFile >> grid[columnLength][rowLength];
            }
            a.push_back(m_row);
            }
            //close the puzzle file
            puzzleFile.close();
            vector<vector<char>> v;
            for (int i = 0; i < 9; ++i) {
        //vector<char> vi(begin(grid[i], end(grid[i])));
        }
            //for loop through the grid
            for (int i = 0; i < 9; i++)
            {
            for (int j = 0; j < 9; j++)
            //print out row and column of the grid
            cout << a[i][j] << " ";
            cout << endl;
            }
            simplePuzzleGrid = a;
        }
        else {
            cout << "[DEBUG] Simple Puzzle file could not be read in" << endl;
        }
}

// This method will then read the dictionary and store the words in the simple dictionary data structure
fn read_simple_dictionary() {
    ifstream file;
    file.open(dictionaryName);
    //if the file is open
    if(file.is_open())
        {
            //while the file is not at the end
            while (!file.eof())
                {
                    simpleResult newWord = simpleResult();
                    getline(file, newWord.value);
                    cout << "Simple dictionary read in is " << newWord.value << endl;
                    newWord.isFound = false;
                    simpleDictionary.push_back(newWord);
                }
            //if reach the end of the dictionary, print a message to indicate as such
            if(file.eof())
                {
                    cout << "[DEBUG] End of the dictionary" << endl;
                }
        }
        else {
            cout << "[DEBUG] Simple Dictionary file could not be read in" << endl;
        }
}

// This method will solve the puzzle using the simple grid data structure
//with the simple dictionary data structure.
fn solve_simple_puzzle() {
    isSimple = true;
    //loop through words in the dictionary
    for (auto& word : simpleDictionary)
        {
            //for loop going 'across' the X axis of the simple grid
            for (int x = 0; x < 9; x++)
            {
            //for loops going 'down' the Y axis
            for (int y = 0; y < 9; y++)
            {
            cellsVisited++;
            // first letter match
            if (simplePuzzleGrid[y][x] == word.value.at(0)) {

            dictEntriesVisited++;
            // construct 8 lines
            vector<string> lines;
            for (int i = 0; i < 8; ++i) {
            string t = getDirLine(word.value.length(), x, y, modifierArray[i][0], modifierArray[i][1]);
            if (t.length() == word.value.length())
            lines.push_back(t);
            }

            for (string& l : lines) {
            word.isFound |= l == word.value;
            }

            if (word.isFound == true) {
            // word is found, log info
            wordsMatched++;

            foundWords.push_back(word.value);
            word.x = x;
            word.y = y;
            goto nextWord;
            }
            }
            }
            }
            nextWord:
            if(foundWords.size() != word.value.size())
            continue;
        }

}

//This method will take a string parameter that will represent the filename of the output file.
//This method will output the results from the previous Solve() method to the given output file,
//which will be either results_simple_puzzle_simple_dictionary.txt
//or results_simple_puzzle_advanced_dictionary.txt or results_advanced_puzzle_simple_dictionary.txt.
fn write_results() {
    ofstream file;

    string simpleSimple = "results_simple_puzzle_simple_dictionary.txt";
    string simpleAdvanced = ("results_simple_puzzle_advanced_dictionary.txt");
    string advancedSimple = ("results_advanced_puzzle_simple_dictionary.txt");


    // look for making output work static
    Output o = Output();
    if(puzzleComplexity == 0 && dictComplexity == 0)
        {
            file.open(simpleSimple);
        }
        else if(puzzleComplexity == 0 && dictComplexity == 1)
            {
                file.open(simpleAdvanced);
            }
            else if(puzzleComplexity == 1 && dictComplexity == 0)
                {
                    file.open(advancedSimple);
                }
    if (isSimple == true)
        {
            if (file.is_open())
                {
                    file << o.numWordsLine(wordsMatched) << endl;

                    file << o.wordsMatched(simpleDictionary) << endl;

                    file << o.wordsUnmatched(simpleDictionary) << endl;

                    file << o.visitedLine(cellsVisited) << endl;

                    file << o.dictEntriesVisited(dictEntriesVisited) << endl;

                    file << o.populateTime(loadTime) << endl;

                    file << o.solveTime(solveTime) << endl;
                }
            file.close();
        }
        else
        {
            if (file.is_open())
                {
                    file << "ENTER ADVANCED OUTPUT HERE" << endl;
                }
            file.close();
        }
}