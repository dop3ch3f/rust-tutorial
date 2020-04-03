// Lifetime elision definition
    // Set of rules in the compiler
        // if rules aren't enough to figure out lifetime parameters, compiler will error
        // Override defaults by adding lifetime parameters
        // Can annotate lifetimes if you want

// Three lifetime elision rules
    // in parameters, each reference gets its own lifetime
    // if theres is one reference in the parameters, the returned reference gets that lifetime
    //(Methods only) if there is a &self or &mut self parameter, returned reference gets that lifetime

// Lifetime elision in previous modules examples