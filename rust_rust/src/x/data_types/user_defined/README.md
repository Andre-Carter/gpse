4. User-defined data types
Rust also gives us the ability to construct our own types.
Type	Purpose	Example
struct	Named data structure	Entity, Vector3
enum	One of several variants	Color, Command
union	Shared storage for fields	Low-level data representation
type alias	Alternative name for a type	type EntityId = u64
trait	Shared behavior contract	Display, Iterator