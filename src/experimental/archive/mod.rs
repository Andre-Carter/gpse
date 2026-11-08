pub mod dictionary;

// pain: dictionlisting synonyms, pronoucation, etc.

pub struct DictionListing {
    word: String,
    definition: String,
}

// perhaps we use a less strict data type than a struct

// word = diction_array[0]
// definition = diction_array[1]
// etc. something a bit more dynamic if we expand 
// the information of a single word we don't have to 
// refactor the entire dictionary codebase

// expand this idea towards archive::nomenclature