//things that needs a fix
- da kine when using cli calc and pressing enter you get a error which is fine but can we handle that or avoid it with refactoring

- conversion cli as of now is useless. check xdoc> planning> roadmap> line: 792 (10/30/26)

Candidates for experiments/

Candidate	                    Why
mathematical/symbolic/runes.rs	TWO_PI = 2.0 + PI is addition, not 2π. The names mislead, so it shouldn’t sit in the trusted tree until they’re settled.
mathematical/carter/combimetrics.rs and carter_cartesian.rs	Your notes call Combimetrics a working name that isn’t formally approved. The Cartesian file doesn’t compile and is commented out of mod.rs.
mathematical/dimensional/*	Commented out of mod.rs, with two competing Dimensions definitions. Experimental, even though it’s central to the Quantity plans.
mathematical/space/cartesian_space.rs	Entirely commented-out code.
systems/command/tools/math.rs (tokenizer)	A parser in progress. It’s wired to the CLI, so the dispatcher exception would apply.
systems/mission/protocols, systems/personnel	They compile now, but decisions.md marks them deferred and exploratory.
lang/, gui/, and the engineering, audio and image additions	The newest breadth, and the ones I’d freeze first. I haven’t seen gui and the others, so I’m going by your description.