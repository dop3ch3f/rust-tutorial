// Lifetime parameters are descriptive
    // - Describe intended relationships between references
    // - Concrete lifetimes that fill in the parameters are determined by the code calling the definitions containing lifetime parameters

// Lifetime parameters are not prescriptive
    // - Don't prescribe how long an input reference or value actually lives
    // - Don't change how long an input reference or value actually lives
    // - Can't use lifetime parameters to set/assign/force a lifetime
    // - Can't make a reference live long enough

// 'static lifetime
    // Only concrete lifetime you can specify
    // References valid during the whole program
    // Similar to static variables
    // References to static variables, constants, literals

// Questions to ask when fixing lifetime-related errors
    // What are the relationships in the implementation
    // What are the concrete lifetimes where the definition is used
    // How do i reogranize the code so the references are always valid
    // Would owned values be more appropriate than references